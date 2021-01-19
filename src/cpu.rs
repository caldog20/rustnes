
#![allow(dead_code)]
use crate::instructions::*;
use crate::bus::BUS;
// const STACK: u16 = 0x0100;
// const STACK_RESET: u8 = 0xFD;
bitflags! {
    pub struct Flags: u8{
    const C = 0b00000001;
    const Z = 0b00000010;
    const I = 0b00000100;
    const D = 0b00001000;
    const B = 0b00010000;
    const U = 0b00000000;
    const V = 0b01000000;
    const N = 0b10000000;
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct Registers {
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8, 
}
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Modes {
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   NoneAddressing,
}

// pub struct CPU {
//     pub pc: u16,
//     pub sp: u8,
//     pub acc: u8,
//     pub status: Flags,
//     pub ix: u8,
//     pub iy: u8,
//     pub bus: BUS,
// }

pub struct CPU {
    pub registers: Registers,
    pub status: Flags,
    pub bus: BUS,
}


pub trait MEM {
    fn read_mem(&self, addr: u16) -> u8;
    
    fn write_mem(&mut self, addr: u16, data: u8);
    
    fn read_mem_u16(&self, addr: u16) -> u16 {
        let lo = self.read_mem(addr) as u16;
        let hi = self.read_mem(addr + 1 ) as u16;
        (hi << 8) | (lo as u16)
    }
    fn write_mem_u16(&mut self, addr: u16, data: u16) {
        let lo = (data & 0xFF) as u8;
        let hi = (data >> 8) as u8;
        self.write_mem(addr, lo);
        self.write_mem(addr + 1, hi);
    }
}
impl MEM for CPU {
    fn read_mem(&self, addr:u16) -> u8 {
        self.bus.read_mem(addr)
    }
    fn write_mem(&mut self, addr: u16, data: u8) {
        self.bus.write_mem(addr, data)
    }
    fn read_mem_u16(&self, addr: u16) -> u16 {
        self.bus.read_mem_u16(addr)
    }
    fn write_mem_u16(&mut self, addr: u16, data: u16) {
        self.bus.write_mem_u16(addr, data)
    }
}

impl Registers {
    pub fn init() -> Self {
        Registers {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::init(),
            status: Flags::from_bits_truncate(0b100100),
            bus: BUS::new(),
        }
    }
    pub fn load_rom(&mut self, program: Vec<u8>) {
        // for i in 0..program.len() {
        //     self.bus.prg_mem[0x0000 + i] = program[i];
        // }
        self.bus.prg_mem[0x0000 .. (0x0000 + program.len())].copy_from_slice(&program[..]);
        self.registers.a = 0x8000;
    }

    pub fn zero_neg_flags(&mut self, result: u8) {
        if result == 0 {
            self.status.insert(Flags::Z);
        } else {
            self.status.remove(Flags::Z);
        }
        if result & 0b1000_0000 != 0 {
            self.status.insert(Flags::N)
        } else {
            self.status.remove(Flags::N)
        }
    }

    pub fn decode(&mut self) {
        // self.pc = 0;
        loop {
            let opcode = self.read_mem(self.registers.pc);
            self.registers.pc += 1;
            print!("OPCODE: {:#04X}\n", opcode);

            match opcode {
                // 0x00 => return,
                0x01 => {
                    print!("Terminated");
                    return;
                }
                0xA9 => {
                    let param = self.read_mem(self.registers.pc);
                    lda(self, param);
                }
                0xAA => {
                    tax(self);
                }
                _ => panic!("NO OPCODE")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_lda_a9() {
        // let bus = BUS::new();
        let mut cpu = CPU::new();
        cpu.load_rom(vec![0xa9, 0x05, 0x01]);
        cpu.decode();
        assert_eq!(cpu.registers.a, 0x05);
        assert!(cpu.status & Flags::Z == Flags::U);
        assert!(cpu.status & Flags::N == Flags::U);
    }
    #[test]
    fn test_lda_a9_zero() {
        // let bus = BUS::new();
        let mut cpu = CPU::new();
        cpu.load_rom(vec![0xa9, 0x00, 0x01]);
        cpu.decode();
        assert!(cpu.status & Flags::Z == Flags::Z);

    }
    #[test]
    fn test_lda_a9_neg() {
        // let bus = BUS::new();
        let mut cpu = CPU::new();
        cpu.load_rom(vec![0xa9, 0xF0, 0x01]);
        cpu.decode();
        assert!(cpu.status & Flags::N == Flags::N);
    }
    #[test]
    fn test_tax() {
        // let bus = BUS::new();
        let mut cpu = CPU::new();
        cpu.load_rom(vec![0xa9, 0x05, 0xaa, 0x00, 0x01]);
        cpu.decode();
        assert_eq!(cpu.registers.x, cpu.registers.a);
        print!("TAX: {:#04X} {:#04X}\n", cpu.registers.x, cpu.registers.a);
    }
    #[test]
    fn test_load_prg() {
        let mut cpu = CPU::new();
        let testrom = vec![0xa9, 0x05, 0xaa, 0x00, 0x01];
        cpu.load_rom(testrom);
        cpu.decode();

    }
}