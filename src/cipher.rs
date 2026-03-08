//! Shellcode XOR encoder binary.
//!
//! Reads a raw shellcode file, encrypts it with a random single-byte XOR key,
//! and writes the encrypted output. The generated key must be set as `XOR_KEY`
//! in `shellcode.rs` before compiling the loader.
//!
//! # Usage
//! ```bash
//! cargo run --bin encoding -- shellcode.bin [output.bin]
//! ```

use clap::{Arg, Command};
use rand::Rng;
use std::fs::File;
use std::io::{self, Read, Write};

fn xor_encrypt(input: &str, output: &str, key: u8) -> io::Result<()> {
    let mut file = File::open(input)?;
    let mut shellcode = Vec::new();
    file.read_to_end(&mut shellcode)?;
    println!("[+] Read {} bytes from '{}'", shellcode.len(), input);

    for byte in shellcode.iter_mut() {
        *byte ^= key;
    }
    println!("[+] Encrypted with key 0x{:02X}", key);

    let mut out = File::create(output)?;
    out.write_all(&shellcode)?;
    println!("[+] Written to '{}'", output);

    Ok(())
}

fn main() {
    let matches = Command::new("RustLoader Encoder")
        .version("0.2.0")
        .author("3xploit666")
        .about("XOR-encrypts shellcode with a random key for use with RustLoader")
        .arg(
            Arg::new("input")
                .help("Path to the raw shellcode file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("Path to the encrypted output file")
                .default_value("encrypted.bin")
                .index(2),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    let key: u8 = rand::thread_rng().gen();
    println!("[*] Random XOR key: 0x{:02X}", key);
    println!("[!] Set XOR_KEY = 0x{:02X} in src/shellcode.rs before building the loader", key);

    if let Err(e) = xor_encrypt(input, output, key) {
        eprintln!("[!] Error: {}", e);
        std::process::exit(1);
    }
}
