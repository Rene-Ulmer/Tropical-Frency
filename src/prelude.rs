// Set up custom memory handler & stuff.
use alloc::boxed::Box;
use alloc::vec::Vec;

#[panic_implementation]
#[no_mangle]
pub fn panic(_panic_info: &::core::panic::PanicInfo) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

#[alloc_error_handler]
#[no_mangle]
pub extern "C" fn oom(_: ::core::alloc::Layout) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

// In order to work with the memory we expose (de)allocation methods
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let mut buf: Vec<u8> = Vec::with_capacity(size);
    unsafe { buf.set_len(size) };
    let b = buf.into_boxed_slice();
    Box::into_raw(b) as *mut u8
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, _cap: usize) {
    unsafe {
        Box::from_raw(ptr);
    }
}
