use std::alloc::{alloc, dealloc, Layout};
use std::mem::MaybeUninit;
use std::ptr::{self, NonNull};


/*
Monomorphization (Monomorfización)

Cuando se usa un tipo genérico `T` en Rust, el compilador genera versiones específicas del código para cada tipo concreto que se usa. Este proceso se llama monomorfización.

de esa manera, si tienes una función genérica como:
fn foo<T>(x: T) { ... }
y la llamas con diferentes tipos:
foo(5);        // T es i32
foo(3.14);     // T es f64
El compilador genera dos versiones de `foo`:
fn foo_i32(x: i32) { ... }
fn foo_f64(x: f64) { ... }
Esto permite que el código genérico sea tan eficiente como el código específico para cada tipo, ya que el compilador puede optimizar cada versión generada. Además estas versiones específicas se crean en tiempo de compilación, no en tiempo de ejecución, lo que significa que no hay sobrecarga adicional al usar genéricos.
*/
pub struct MyVec<T> {
    ptr: NonNull<MaybeUninit<T>>,
    capacity: usize,
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            capacity: 0,
            len: 0,
        }
    }

    /*
    Asigna memoria sin inicializar para almacenar `cap` elementos de tipo `T`.

    Esta función solicita memoria directamente al allocador del sistema usando `std::alloc`,
    sin depender de Vec o ninguna otra estructura de datos de la biblioteca estándar.

    # Parámetros
    - `cap`: Capacidad (número de elementos) para la cual se asignará memoria

    # Proceso
    1. Calcula el `Layout` (tamaño y alineación) para `cap` elementos de tipo `MaybeUninit<T>`
    2. Solicita memoria al sistema operativo usando `alloc()`
    3. Convierte el puntero raw (`*mut u8`) a `*mut MaybeUninit<T>`
    4. Verifica que la asignación fue exitosa (puntero no-null)
    5. Guarda el puntero y capacidad en la estructura

    # Panics
    - Si `cap == 0` (no se permite asignar capacidad 0)
    - Si el cálculo de tamaño causa overflow aritmético
    - Si la asignación de memoria falla (sistema sin memoria)

    # Safety
    La memoria asignada está completamente sin inicializar (contiene basura).
    Se usa `MaybeUninit<T>` para indicar que los valores no han sido inicializados.
    Esta memoria DEBE ser liberada con `dealloc()` cuando el vector sea destruido.


    ---

    Conceptos fundamentales de cómo se organiza la memoria:

    Tamaño (Size)
        El tamaño es cuántos bytes ocupa un tipo en memoria. Ejemplos:
            u8: 1 byte
            u32: 4 bytes
            u64: 8 bytes
            (u8, u32): 8 bytes (no siempre es la suma directa, ver alineación)
            std::mem::size_of::<u32>()  // retorna 4
    Alineación (Alignment)
        La alineación es la dirección de memoria donde un tipo debe comenzar. Es un requisito del hardware. Regla: Un tipo debe estar en una dirección de memoria que sea múltiplo de su alineación.
        Ejemplos:
            u8: alineación de 1 → puede estar en cualquier dirección (0, 1, 2, 3, 4...)
            u32: alineación de 4 → debe estar en dirección múltiplo de 4 (0, 4, 8, 12...)
            u64: alineación de 8 → debe estar en dirección múltiplo de 8 (0, 8, 16, 24...)
            std::mem::align_of::<u32>()  // retorna 4

    ¿Por qué existe la alineación?
        - Los procesadores modernos leen memoria en "chunks" (bloques). Si un dato no está alineado correctamente:
        - El CPU necesita hacer múltiples lecturas (más lento)
        - En algunos procesadores, causa un crash

    Ejemplo visual:
    Memoria (direcciones):
    ┌──┬──┬──┬──┬──┬──┬──┬──┐
    │0 │1 │2 │3 │4 │5 │6 │7 │
    └──┴──┴──┴──┴──┴──┴──┴──┘

    ✅ u32 bien alineado (empieza en 0 o 4):
    ┌─────────┬─────────┐
    │ u32 (4) │ u32 (4) │
    └─────────┴─────────┘
    0  1  2  3  4  5  6  7

    ❌ u32 mal alineado (empieza en 1):
        ┌─────────┐
        │ u32 (4) │  ← cruza la frontera!
        └─────────┘
    0  1  2  3  4  5  6  7


    En la función allocate:

    let layout = Layout::array::<MaybeUninit<T>>(cap).unwrap();
    Layout calcula:
        - Tamaño total: size_of::<T>() × cap bytes necesarios
        - Alineación: La alineación que requiere el tipo T

    Esto le dice al allocador: "Dame memoria de X bytes que empiece en una dirección múltiplo de Y"

    */
    /// Asigna un nuevo bloque de memoria para `cap` elementos.
    ///
    /// Retorna un `NonNull` apuntando al nuevo bloque de memoria sin inicializar.
    /// Esta es una función auxiliar usada por `allocate` y `grow`.
    fn allocate_raw(cap: usize) -> NonNull<MaybeUninit<T>> {
        assert!(cap > 0);
        let layout = Layout::array::<MaybeUninit<T>>(cap).unwrap();
        let raw_ptr = unsafe { alloc(layout) } as *mut MaybeUninit<T>;
        NonNull::new(raw_ptr).expect("allocation failed")
    }

    fn allocate(&mut self, cap: usize) {
        assert!(cap > 0);

        /*
        1. Calcula size y alignment
            Layout::array::<T>(cap)
            Internamente hace:
                - size = size_of::<T>() * cap
                - align = align_of::<T>()
        2. Verifica overflow aritmético
            let layout = Layout::array::<u64>(usize::MAX)?;
                ❌ Retorna Err porque usize::MAX * 8 causa overflow
        3. Crea una estructura Layout que alloc() necesita
            pub struct Layout {
                size: usize,
                align: usize,
            }
            alloc() requiere un Layout, no puedes pasarle size y align separados.

        ---

        ✅ Con Layout::array (SEGURO)
        let layout = Layout::array::<MaybeUninit<T>>(cap).unwrap();
        let raw_ptr = unsafe { alloc(layout) } as *mut MaybeUninit<T>;

        ❌ Sin Layout (NO COMPILA - alloc() necesita un Layout)
        let size = std::mem::size_of::<MaybeUninit<T>>() * cap;
        let align = std::mem::align_of::<MaybeUninit<T>>();
        let raw_ptr = unsafe { alloc(size, align) }; // ❌ Error: alloc toma Layout, no (size, align)

        🟡 Creando Layout manualmente (POSIBLE pero innecesario)
        let size = std::mem::size_of::<MaybeUninit<T>>()
            .checked_mul(cap)
            .expect("overflow");
        let align = std::mem::align_of::<MaybeUninit<T>>();
        let layout = Layout::from_size_align(size, align).unwrap();
        let raw_ptr = unsafe { alloc(layout) } as *mut MaybeUninit<T>;

        */
        self.ptr = Self::allocate_raw(cap);
        self.capacity = cap;
    }

    /*
    Aumenta la capacidad del vector cuando se queda sin espacio.

    Duplica la capacidad actual (o usa 4 si la capacidad es 0), asigna un nuevo
    bloque de memoria, copia todos los elementos existentes al nuevo bloque,
    y libera el bloque viejo.

    # Proceso
        1. Calcula la nueva capacidad (doble de la actual, o 4 si es 0)
        2. Asigna un nuevo bloque de memoria con la nueva capacidad
        3. Copia todos los elementos del bloque viejo al nuevo usando `ptr::copy_nonoverlapping`
        4. Libera el bloque viejo con `dealloc` (si la capacidad vieja > 0)
        5. Actualiza el puntero y capacidad a los nuevos valores

    # Panics
        - Si la asignación del nuevo bloque falla
        - Si el cálculo de la nueva capacidad causa overflow

    # Safety
        - Usa operaciones unsafe para copiar memoria y liberar el bloque viejo.
        - Los elementos copiados mantienen su estado de inicialización (MaybeUninit).
    */
    fn grow(&mut self) {
        // Calcula nueva capacidad: duplica la actual, o 4 si es 0
        let new_cap = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };

        // Asigna el nuevo bloque de memoria reutilizando allocate_raw
        let new_ptr = Self::allocate_raw(new_cap);

        // Copia los elementos del bloque viejo al nuevo (si hay elementos)
        if self.capacity > 0 {
            unsafe {
                // copy_nonoverlapping copia `len` elementos de src a dst
                // Es seguro porque los bloques no se superponen

                /*
                Aqui es importante mencionar que podríamos hacer un loop manualmente
                para copiar cada elemento pero en terminos de rendimiento
                ptr::copy_nonoverlapping es mucho más rápido porque hace todo en
                paralelo a nivel de memoria.


                for i in 0..self.len {
                    El compilador debe:
                    - Verificar el índice i en cada iteración
                    - Calcular offset (i * size_of::<T>())
                    - Copiar 1 elemento a la vez
                }
                */
                ptr::copy_nonoverlapping(
                    self.ptr.as_ptr(),  // src: puntero al bloque viejo
                    new_ptr.as_ptr(),   // dst: puntero al bloque nuevo
                    self.len,           // count: número de elementos a copiar
                );

                // Libera el bloque viejo
                let old_layout = Layout::array::<MaybeUninit<T>>(self.capacity).unwrap();
                dealloc(self.ptr.as_ptr() as *mut u8, old_layout);
            }
        }

        // Actualiza el puntero y capacidad
        self.ptr = new_ptr;
        self.capacity = new_cap;
    }

    /// Añade un elemento al final del vector.
    ///
    /// Si el vector está lleno (len == capacity), primero crece la capacidad
    /// usando `grow()` antes de añadir el elemento.
    ///
    /// # Proceso
    /// 1. Verifica si hay espacio disponible (len < capacity)
    /// 2. Si no hay espacio, llama a `grow()` para duplicar la capacidad
    /// 3. Calcula la posición donde escribir el nuevo elemento (ptr + len)
    /// 4. Escribe el elemento usando `ptr::write`
    /// 5. Incrementa `len`
    ///
    /// # Parámetros
    /// - `new_elem`: El elemento de tipo `T` a añadir al final
    ///
    /// # Safety
    /// Usa `ptr::write` para escribir en memoria sin inicializar.
    /// El elemento se mueve (move) al vector, transfiriendo la propiedad.
    pub fn push_back(&mut self, new_elem: T) {
        // Si no hay espacio, crece el vector
        if self.len >= self.capacity {
            self.grow();
        }

        unsafe {
            // Calcula la dirección donde escribir: ptr + len
            // add(len) avanza el puntero len posiciones (ptr + len * size_of::<T>())
            let dst = self.ptr.as_ptr().add(self.len);

            // Escribe el elemento en memoria sin inicializar
            // write() mueve new_elem a la ubicación dst sin llamar al destructor del valor anterior
            ptr::write(dst, MaybeUninit::new(new_elem));
        }

        // Incrementa la longitud
        self.len += 1;
    }

    /// Obtiene una referencia inmutable al elemento en la posición `index`.
    ///
    /// Retorna `Some(&T)` si el índice es válido, o `None` si está fuera de rango.
    ///
    /// # Complejidad
    /// **O(1)** - Acceso en tiempo constante usando aritmética de punteros.
    ///
    /// # Proceso
    /// 1. Verifica que `index < len` (bounds checking)
    /// 2. Calcula la dirección: `ptr + index * size_of::<T>()`
    /// 3. Lee la referencia del elemento
    ///
    /// # Parámetros
    /// - `index`: Posición del elemento a obtener (0-indexed)
    ///
    /// # Ejemplos
    /// ```ignore
    /// let mut v = MyVec::new();
    /// v.push_back(10);
    /// v.push_back(20);
    /// v.push_back(30);
    ///
    /// assert_eq!(v.get(0), Some(&10));
    /// assert_eq!(v.get(1), Some(&20));
    /// assert_eq!(v.get(5), None);  // Fuera de rango
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        // Verificación de límites
        if index >= self.len {
            return None;
        }

        unsafe {
            // Calcula la dirección del elemento:
            // dirección = ptr + (index × size_of::<T>())
            // Esto es O(1): una simple operación matemática
            let element_ptr = self.ptr.as_ptr().add(index);

            // Convierte MaybeUninit<T> a T
            // assume_init_ref() asume que el elemento está inicializado
            // (sabemos que lo está porque index < len)
            Some((*element_ptr).assume_init_ref())
        }
    }

    /// Obtiene una referencia mutable al elemento en la posición `index`.
    ///
    /// Retorna `Some(&mut T)` si el índice es válido, o `None` si está fuera de rango.
    ///
    /// # Complejidad
    /// **O(1)** - Acceso en tiempo constante.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        unsafe {
            let element_ptr = self.ptr.as_ptr().add(index);
            Some((*element_ptr).assume_init_mut())
        }
    }

    /// Retorna la longitud actual del vector (número de elementos).
    pub fn len(&self) -> usize {
        self.len
    }

    /// Retorna la capacidad del vector (espacio total asignado).
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Retorna `true` si el vector no contiene elementos.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

fn main() {
    println!("MyVec implementation - run 'cargo test' to see tests");
}
