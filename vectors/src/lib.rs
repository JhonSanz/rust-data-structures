use std::alloc::{alloc, dealloc, Layout};
use std::mem::MaybeUninit;
use std::ptr::{self, NonNull};

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

    /// Asigna un nuevo bloque de memoria para `cap` elementos.
    ///
    /// Retorna un `NonNull` apuntando al nuevo bloque de memoria sin inicializar.
    /// Esta es una función auxiliar usada por `grow`.
    fn allocate_raw(cap: usize) -> NonNull<MaybeUninit<T>> {
        assert!(cap > 0);
        let layout = Layout::array::<MaybeUninit<T>>(cap).unwrap();
        let raw_ptr = unsafe { alloc(layout) } as *mut MaybeUninit<T>;
        NonNull::new(raw_ptr).expect("allocation failed")
    }

    /// Aumenta la capacidad del vector cuando se queda sin espacio.
    fn grow(&mut self) {
        let new_cap = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };

        let new_ptr = Self::allocate_raw(new_cap);

        if self.capacity > 0 {
            unsafe {
                ptr::copy_nonoverlapping(
                    self.ptr.as_ptr(),
                    new_ptr.as_ptr(),
                    self.len,
                );

                let old_layout = Layout::array::<MaybeUninit<T>>(self.capacity).unwrap();
                dealloc(self.ptr.as_ptr() as *mut u8, old_layout);
            }
        }

        self.ptr = new_ptr;
        self.capacity = new_cap;
    }

    /// Añade un elemento al final del vector.
    pub fn push_back(&mut self, new_elem: T) {
        if self.len >= self.capacity {
            self.grow();
        }

        unsafe {
            let dst = self.ptr.as_ptr().add(self.len);
            ptr::write(dst, MaybeUninit::new(new_elem));
        }

        self.len += 1;
    }

    /// Obtiene una referencia inmutable al elemento en la posición `index`.
    ///
    /// # Complejidad
    /// **O(1)** - Acceso en tiempo constante usando aritmética de punteros.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        unsafe {
            let element_ptr = self.ptr.as_ptr().add(index);
            Some((*element_ptr).assume_init_ref())
        }
    }

    /// Obtiene una referencia mutable al elemento en la posición `index`.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        unsafe {
            let element_ptr = self.ptr.as_ptr().add(index);
            Some((*element_ptr).assume_init_mut())
        }
    }

    /// Retorna la longitud actual del vector.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Retorna la capacidad del vector.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Retorna `true` si el vector no contiene elementos.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
