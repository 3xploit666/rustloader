# RustLoader

<p align="center">
  <img src="assets/rustloader.jpg">

</p>

![POC Demo](assets/poc.gif)
## 📜 Descripción
**RustLoader** es un sofisticado cargador de malware desarrollado en Rust, orientado a la investigación y la demostración de técnicas de evasión y ejecución sigilosa. El diseño está centrado en la inyección y ejecución de shellcode cifrado, manejando la memoria de forma directa para evitar las APIs de alto nivel que son fácilmente monitoreables.

## 🌟 Características Destacadas
- **Detección de Depuradores**: Implementa técnicas avanzadas para detectar depuradores y prevenir ejecución en entornos monitoreados.
- **Simulación de Interacción Humana**: Requiere múltiples clics de mouse para simular la presencia y interacción de un usuario antes de proceder.
- **Gestión Avanzada de Memoria**: Gestiona la memoria con llamadas a bajo nivel para asignación y borrado, minimizando la visibilidad ante herramientas de monitoreo.
- **Ejecución de Shellcode Cifrado**: Ejecuta shellcode cifrado directamente desde la memoria, usando técnicas de desencriptación en tiempo real.

## 🔧 Prerrequisitos
![Rust Badge](https://img.shields.io/badge/rust-stable-brightgreen.svg)
![Windows Badge](https://img.shields.io/badge/windows-10-blue.svg)

- **Rust**: Última versión estable.
- **Microsoft Visual C++ Build Tools**: Esencial para la compilación en Windows.

## 📂 Estructura del Proyecto
```plaintext
src/
│
├── main.rs          - Inicia los procedimientos de seguridad y carga del malware.
├── patch.rs         - Parchea procesos en ejecución para técnicas de persistencia.
├── shellcode.rs     - Carga y ejecuta el shellcode.
└── utils.rs         - Utilidades como simulación de clics y esperas.
└── cipher.rs         - genera el payload encodeado con simple xor.
    
```

## 🚀 Uso Encoding payload
```plaintext
cargo run --bin encoding demon.x64.bin


```
Random XOR key generated: 0x60

Successfully read shellcode from '.\demon.x64.bin'

Successfully encrypted shellcode with key '0x60'

Successfully wrote encrypted shellcode to 'encrypted.bin'

## 🔥 Testing loader
```plaintext

cargo run --bin loader 

```



## ⚠️ Disclaimer

Este código está destinado exclusivamente para uso educativo y de investigación. No es adecuado para uso en producción ni para realizar actividades ilegales. Los desarrolladores declinan toda responsabilidad por el uso indebido de este software.




