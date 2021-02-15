use std::{mem, ptr, vec};

#[repr(C)]
pub struct Vec<T> {
    pub ptr: ptr::NonNull<T>,
    pub len: usize,
    pub cap: usize,
}

impl<T> From<vec::Vec<T>> for Vec<T> {
    #[inline]
    fn from(vec: vec::Vec<T>) -> Self {
        let len = vec.len();
        let cap = vec.capacity();
        let ptr = mem::ManuallyDrop::new(vec).as_mut_ptr();
        Self {
            ptr: unsafe {
                // Safety: `Vec` guarantees its pointer is nonnull.
                ptr::NonNull::new_unchecked(ptr)
            }
            .into(),
            len,
            cap,
        }
    }
}

impl From<&str> for Vec<u8> {
    fn from(s: &str) -> Self {
        let vec: vec::Vec<u8> = vec::Vec::from(s);
        Vec::from(vec)
    }
}

impl From<String> for Vec<u8> {
    fn from(s: String) -> Self {
        Vec::from(s.as_str())
    }
}
