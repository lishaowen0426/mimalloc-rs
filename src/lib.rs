#![no_std]
#![allow(dead_code)]

use core::marker::{PhantomData, PhantomPinned};
use libc::{c_void, size_t};

#[repr(C)]
pub struct MiHeap {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}
#[link(name = "mimalloc", kind = "static")]
extern "C" {
    fn mi_calloc(count: size_t, size: size_t) -> *mut c_void;
    fn mi_heap_new() -> *mut MiHeap;

}

pub fn sys_alloc(count: usize, size: usize, _is_unsafe: bool) -> *mut u8 {
    unsafe { mi_calloc(count as size_t, size as size_t) as *mut u8 }
}
