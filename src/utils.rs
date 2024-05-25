use std::time::Duration;
use std::thread::sleep;
use winapi::ctypes::c_void;

pub fn sleep_millis(millis: u64) {
    sleep(Duration::from_millis(millis));
}

pub(crate) unsafe fn xor_encrypt_decrypt(key: u8, ptr: *mut c_void, len: usize) {
    println!("Iniciando operación XOR con clave: 0x{:02X}", key);
    let slice = std::slice::from_raw_parts_mut(ptr as *mut u8, len);
    for byte in slice.iter_mut() {
        *byte ^= key;
    }
    println!("Operación XOR completada.");
}
