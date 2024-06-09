use core::marker::{PhantomData, PhantomPinned};
use libc::{c_void, size_t};
use std::mem::size_of;

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

fn main() {
    println!("Hello, world!");
    unsafe {
        let p = mi_calloc(1, size_of::<u8>()) as *mut u8;
        println!("p: {:#x}", p as usize);

        let heap = mi_heap_new();
        println!("head: {:#x}", heap as usize);
    }
}
