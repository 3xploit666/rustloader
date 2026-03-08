//! Utility functions for the loader.

use std::thread::sleep;
use std::time::Duration;
use winapi::ctypes::c_void;

/// Sleeps for the specified number of milliseconds.
pub fn sleep_millis(millis: u64) {
    sleep(Duration::from_millis(millis));
}

/// Decrypts a memory region in-place using single-byte XOR.
///
/// # Safety
/// `ptr` must point to a valid, writable allocation of at least `len` bytes.
pub unsafe fn xor_decrypt(key: u8, ptr: *mut c_void, len: usize) {
    let slice = std::slice::from_raw_parts_mut(ptr as *mut u8, len);
    for byte in slice.iter_mut() {
        *byte ^= key;
    }
}
