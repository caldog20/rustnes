#![allow(dead_code)]
pub mod cpu;
pub mod bus;
pub mod nes;
pub mod instructions;
pub mod opcodes;
use crate::nes::NES;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;





fn main() {
    let mut nes = NES::new();
    let testrom = vec![0xc4, 0x00, 0x01];
    nes.cpu.load_rom(testrom);
    nes.cpu.reset();
    nes.cpu.decode();
}
