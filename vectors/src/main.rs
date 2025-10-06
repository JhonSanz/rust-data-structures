use std::alloc::{alloc, dealloc, Layout};
use std::mem::MaybeUninit;
use std::ptr::NonNull;

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

        // ✅ Con Layout::array (SEGURO)
        let layout = Layout::array::<MaybeUninit<T>>(cap).unwrap();
        let raw_ptr = unsafe { alloc(layout) } as *mut MaybeUninit<T>;

        // ❌ Sin Layout (NO COMPILA - alloc() necesita un Layout)
        let size = std::mem::size_of::<MaybeUninit<T>>() * cap;
        let align = std::mem::align_of::<MaybeUninit<T>>();
        let raw_ptr = unsafe { alloc(size, align) }; // ❌ Error: alloc toma Layout, no (size, align)

        // 🟡 Creando Layout manualmente (POSIBLE pero innecesario)
        let size = std::mem::size_of::<MaybeUninit<T>>()
            .checked_mul(cap)
            .expect("overflow");
        let align = std::mem::align_of::<MaybeUninit<T>>();
        let layout = Layout::from_size_align(size, align).unwrap();
        let raw_ptr = unsafe { alloc(layout) } as *mut MaybeUninit<T>;

        */
        let layout = Layout::array::<MaybeUninit<T>>(cap).unwrap();
        let raw_ptr = unsafe { alloc(layout) } as *mut MaybeUninit<T>;
        self.ptr = NonNull::new(raw_ptr).expect("allocation failed");
        self.capacity = cap;
    }
}
