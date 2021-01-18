
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

pub struct CPU {
    pub pc: u16,
    pub sp: u8,
    pub acc: u8,
    pub status: Flags,
    pub ix: u8,
    pub iy: u8,
    pub bus: BUS,
}
pub trait MEM {
    fn read_mem(&self, addr: u16) -> u8;
    fn write_mem(&mut self, addr: u16, data: u8);


}
impl MEM for CPU {
    fn read_mem(&self, addr:u16) -> u8 {
        self.bus.read_mem(addr)
    }
    fn write_mem(&mut self, addr: u16, data: u8) {
        self.bus.write_mem(addr, data)
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            sp: 0,
            acc: 0,
            status: Flags::from_bits_truncate(0b100100),
            ix: 0,
            iy: 0,
            bus: BUS::new(),   
        }
    }
    pub fn load_rom(&mut self, program: Vec<u8>) {
        // for i in 0..program.len() {
        //     self.bus.prg_mem[0x0000 + i] = program[i];
        // }
        self.bus.prg_mem[0x0000 .. (0x0000 + program.len())].copy_from_slice(&program[..]);
        self.pc = 0x8000;
    }

    pub fn zero_carry_flags(&mut self, result: u8) {
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
            let opcode = self.read_mem(self.pc);
            self.pc += 1;
            print!("OPCODE: {:#04X}\n", opcode);

            match opcode {
                // 0x00 => return,
                0x01 => {
                    print!("Terminated");
                    return;
                }
                0xA9 => {
                    let param = self.read_mem(self.pc);
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
        assert_eq!(cpu.acc, 0x05);
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
        assert_eq!(cpu.ix, cpu.acc);
        print!("TAX: {:#04X} {:#04X}\n", cpu.ix, cpu.acc);
    }
    #[test]
    fn test_load_prg() {
        let mut cpu = CPU::new();
        let testrom = vec![0xa9, 0x05, 0xaa, 0x00, 0x01];
        cpu.load_rom(testrom);
        cpu.decode();

    }
}