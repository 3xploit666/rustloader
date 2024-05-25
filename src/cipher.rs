use clap::{Arg, Command};
use rand::Rng; // Añadir para generación de números aleatorios
use std::fs::File;
use std::io::{self, Read, Write};

fn xor_encrypt_shellcode(filename: &str, output_filename: &str, key: u8) -> io::Result<()> {
    // Leer el archivo de shellcode
    let mut file = File::open(filename)?;
    let mut shellcode = Vec::new();
    file.read_to_end(&mut shellcode)?;
    println!("Successfully read shellcode from '{}'", filename);

    // Cifrar el shellcode
    for byte in shellcode.iter_mut() {
        *byte ^= key;
    }
    println!("Successfully encrypted shellcode with key '0x{:02X}'", key);

    // Escribir el shellcode cifrado en un nuevo archivo
    let mut output_file = File::create(output_filename)?;
    output_file.write_all(&shellcode)?;
    println!("Successfully wrote encrypted shellcode to '{}'", output_filename);

    Ok(())
}

fn main() {
    let matches = Command::new("Shellcode Encryptor")
        .version("1.0")
        .author("3xploit666")
        .about("Encrypts a shellcode file using XOR encryption")
        .arg(Arg::new("filename")
            .help("The path to the input shellcode file")
            .required(true)
            .index(1))
        .arg(Arg::new("output_filename")
            .help("The path to the output encrypted file")
            .default_value("encrypted.bin")
            .index(2))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let output_filename = matches.value_of("output_filename").unwrap();

    // Generar una clave XOR aleatoria
    let key: u8 = rand::thread_rng().gen();
    println!("Random XOR key generated: 0x{:02X}", key); // Imprimir la clave en formato hexadecimal

    if let Err(e) = xor_encrypt_shellcode(filename, output_filename, key) {
        eprintln!("Error: {}", e);
    }
}
