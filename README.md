# lm3s6965evb (Cortex-M3) OS Practice
This repository contains practice code for developing an operating system (OS) specifically for the **lm3s6965evb** board.
The board utilizes a **Cortex-M3** processor, and the development is facilitated through the use of **QEMU** emulator.

## References
[jserv/mini-arm-os](https://github.com/jserv/mini-arm-os)  
[Rust Embedded Book](https://docs.rust-embedded.org/embedonomicon/)  

## Prerequisites
- **QEMU**
- **Toolchain**: Follow the instructions in the Rust Embedded Book to set up the necessary tools.

## Some useful commands
```
cargo build
# test with gdb
qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -gdb tcp::3333 -S -nographic -kernel target/thumbv7m-none-eabi/debug/os
# test without gdb
qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -kernel target/thumbv7m-none-eabi/debug/os
# Disassembly
arm-none-eabi-objdump.exe -S .\target\thumbv7m-none-eabi\debug\os > os.txt

# gdb commands
arm-none-eabi-gdb.exe -q target/thumbv7m-none-eabi/debug/os
target remote :3333
```
