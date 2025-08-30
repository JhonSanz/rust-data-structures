use std::mem::MaybeUninit;
use std::ptr;

pub struct MyVec<T> {
    buf: Box<[MaybeUninit<T>]>,
    capacity: usize,
    len: usize,
}

impl<T> MyVec<T> {
    /// Crea un nuevo vector con capacidad inicial
    pub fn new() -> Self {
        let capacity = 4;
        let buf = Self::allocate_buffer(capacity);
        Self {
            buf,
            capacity,
            len: 0,
        }
    }

    fn allocate_buffer(cap: usize) -> Box<[MaybeUninit<T>]> {
        let mut v = Vec::with_capacity(cap);
        let ptr = v.as_mut_ptr();
        std::mem::forget(v); // no liberar
        unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(
                ptr as *mut MaybeUninit<T>,
                cap,
            ))
        }
    }
}
