use std::ptr::null_mut;

use flecsys::{ecs_vec_fini, ecs_vec_init, ecs_vec_t};
/// Wrapper around ecs_vec_t
/// This is *not* ffi compatible. We store `element_size` as an additional field for safety
pub struct EcsVec {
    /// Pointer to the array containing elements
    array: *mut std::ffi::c_void,
    /// Number of elements in the array
    count: u32,
    /// Total capacity of the array in number of elements
    capacity: u32,
    /// Size of each element in bytes
    element_size: u32,
}
impl EcsVec {
    fn into_c(&self) -> ecs_vec_t {
        ecs_vec_t {
            array: self.array,
            count: self.count as _,
            size: self.capacity as _,
        }
    }
    fn from_c(&mut self, vec: ecs_vec_t) {
        self.array = vec.array;
        self.count = vec.count.try_into().unwrap();
        self.capacity = vec.size.try_into().unwrap();
    }
    /// Create a new `EcsVec` with the given size
    /// `elem_size` is the size of each element in bytes
    /// `elem_count` is the number of elements for which you want to pre-allocate memory
    pub fn new(elem_size: u32, elem_count: u32) -> Self {
        let mut new_self = Self::default();
        new_self.element_size = elem_size;
        let mut vec = new_self.into_c();
        unsafe {
            // size/count should fit inside i32
            assert!(elem_count <= i32::MAX as u32);
            assert!(elem_size <= i32::MAX as u32);
            ecs_vec_init(
                null_mut(),
                &mut vec as *mut ecs_vec_t,
                elem_size as _,
                elem_count as _,
            );
        }
        new_self.from_c(vec);
        new_self
    }
}
impl Default for EcsVec {
    fn default() -> Self {
        Self {
            array: std::ptr::null_mut(),
            count: 0,
            capacity: 0,
            element_size: 0,
        }
    }
}

impl Drop for EcsVec {
    fn drop(&mut self) {
        unsafe {
            ecs_vec_fini(
                null_mut(),
                &mut self.into_c() as *mut ecs_vec_t,
                self.element_size as i32,
            );
        }
    }
}
