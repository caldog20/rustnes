//TODO: Clean up matches, figure out cycles, fix program counter increments in gen purpose function.

#![allow(dead_code)]
use crate::instructions::*;
use crate::bus::BUS;
use crate::opcodes::OPCODES;


bitflags! {
    pub struct Flags: u8{
    const C = 0b00000001;
    const Z = 0b00000010;
    const I = 0b00000100;
    const D = 0b00001000;
    const B = 0b00010000;
    const U = 0b00100000;
    const V = 0b01000000;
    const N = 0b10000000;
    const O = 0b00000000;
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

pub enum InterruptType {
    IRQ,
    NMI,
    BRK,
}


#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum Modes {
   Immediate,
   ZeroPage,
   ZeroPageX,
   ZeroPageY,
   Absolute,
   AbsoluteX,
   AbsoluteY,
   IndirectX,
   IndirectY,
   NoneAddressing,
}

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
        for i in 0..program.len() as u16 {
            self.write_mem(0x0600 + i, program[i as usize]);
        }
        // self.bus.prg_mem[0x0000 .. (0x0000 + program.len())].copy_from_slice(&program[..]);
        // self.write_mem_u16(0xFFFC, 0x8000);
    }

    pub fn reset(&mut self) {
        self.registers.a = 0;
        self.registers.x = 0;
        self.status = Flags::from_bits_truncate(0b100100);
        self.registers.pc = 0x600;
    }

    pub fn run(&mut self, program: Vec<u8>) {
        self.load_rom(program);
        self.reset();
        self.decode();
    }

    pub fn get_address(&self, mode: &Modes) -> u16 {
        match mode {
            Modes::Immediate => self.registers.pc,
            _ => self.get_addr_mode(mode, self.registers.pc)
        }
    }

    pub fn get_addr_mode(&self, mode: &Modes, addr: u16) -> u16 {
        match mode {
            Modes::Absolute => self.read_mem_u16(addr),

            Modes::AbsoluteX => {
                let operand = self.read_mem_u16(addr);
                return operand.wrapping_add(self.registers.x as u16)
            }
            Modes::AbsoluteY => {
                let operand = self.read_mem_u16(addr);
                return operand.wrapping_add(self.registers.y as u16)
            }
            Modes::ZeroPage => self.read_mem(addr) as u16,

            Modes::ZeroPageX => { 
                let operand = self.read_mem(addr);
                return operand.wrapping_add(self.registers.y) as u16
            }
            Modes::ZeroPageY => {
                let operand = self.read_mem(addr);
                return operand.wrapping_add(self.registers.x) as u16 
            }
            Modes::IndirectX => {
                let operand = self.read_mem(addr); // Get u8 byte from memory
                let index: u8 = operand.wrapping_add(self.registers.x); // Since read_mem returns u8, cast index as u8 and add register X to operand
                let lo = self.read_mem(index as u16); // lo bit is read from memory at index location
                let hi = self.read_mem(index.wrapping_add(1) as u16); // hi bit is read from the index + 1 location
                (hi as u16) << 8 | lo as u16 // return as BIG ENDIAN
            }
            Modes::IndirectY => {
                let index = self.read_mem(addr);
                let lo = self.read_mem(index as u16);
                let hi = self.read_mem((index as u8).wrapping_add(1) as u16);
                let base = (hi as u16) << 8 | lo as u16;
                return base.wrapping_add(self.registers.y as u16)
            }

            _ => panic!("Mode not supported: {:?}", mode)
        }
    }

    pub fn zero_neg_flags(&mut self, result: u8) {
        if result == 0 {
            self.update_zero_flag(true);
            println!("SET ZERO FLAG");
        } else {
            self.update_zero_flag(false);
            println!("UNSET ZERO FLAG");
        }
        if result >> 7 == 1 {
            self.update_negative_flag(true);
            println!("SET NEG FLAG");
        } else {
            self.update_negative_flag(false);
            println!("UNSET NEG FLAG");
        }
    }
    pub fn update_zero_flag(&mut self, set: bool) {
        match set {
            true => self.status.insert(Flags::Z),
            false => self.status.remove(Flags::Z),
        }
    }
    pub fn update_negative_flag(&mut self, set: bool) {
        match set {
            true => self.status.insert(Flags::N),
            false => self.status.remove(Flags::N),
        }
    }
    pub fn update_carry_flag(&mut self, set: bool) {
        match set {
            true => self.status.insert(Flags::C),
            false => self.status.remove(Flags::C),
        }
    }
    pub fn update_overflow_flag(&mut self, set: bool) {
        match set {
            true => self.status.insert(Flags::V),
            false => self.status.remove(Flags::V),
        }
    }
    pub fn decode(&mut self) {
        // self.pc = 0;
        loop {
            let opcode = self.read_mem(self.registers.pc); // Get Opcode from memory at current program counter location
            self.registers.pc += 1; // increment program counter to next address for instructions
            print!("OPCODE: {:#04X}\n", opcode);
            let mode = OPCODES.get(&opcode).unwrap();
            match opcode {
                0x00 => {
                    println!("BRK");
                    return;
                }
                0x01 => {
                    println!("Skip for testing");
                    self.registers.pc += 2;
                }
                // BIT
                0x24 | 0x2C => {
                    bits(self, &mode);
                    if opcode == 0x2C {
                        self.registers.pc += 2;
                    } else {
                        self.registers.pc += 1;
                    }
                }
                // AND
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    and(self, &mode);
                    match opcode {
                        0x2D | 0x3D | 0x39 => self.registers.pc += 2,
                        _ => self.registers.pc += 1
                    }
                }
                // LDA 
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    lda(self, &mode);
                    match opcode {
                        0xAD | 0xBD | 0xB9 => self.registers.pc += 2, // Absolute mode uses a 2 byte address, so increment program counter twice to skip second value of 2byte address for next opcode
                        _ => self.registers.pc += 1
                    }
                }
                // TAX 
                0xAA => {
                    tax(self);
                    self.registers.pc += 1;
                }
                // ADC
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    adc(self, &mode);
                    match opcode {
                        0x6D | 0x7D | 0x79 => self.registers.pc += 2,
                        _ => self.registers.pc += 1
                    }
                }
                // SBC
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    sbc(self, &mode);
                    match opcode {
                        0xED | 0xFD | 0xF9 => self.registers.pc += 2,
                        _ => self.registers.pc += 1
                    }
                }
                // ASL_ACC
                0x0A => asl_acc(self),
                // ASL
                0x06 | 0x16 | 0x0E | 0x1E => {
                    asl(self, &mode);
                    match opcode {
                        0x0E | 0x1E => self.registers.pc += 2,
                        _ => self.registers.pc += 1
                    }
                }
                // CMP
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    compare(self, &mode, self.registers.a);
                    match opcode {
                        0xCD | 0xDD | 0xD9 => self.registers.pc += 2,
                        _ => self.registers.pc += 1
                    }
                }
                // CMP X
                0xE0 | 0xE4 | 0xEC => {
                    compare(self, &mode, self.registers.x);
                    if opcode == 0xEC {
                        self.registers.pc += 2;
                    } else {
                        self.registers.pc += 1;
                    }
                }
                // CMP Y
                0xC0 | 0xC4 | 0xCC => {
                    compare(self, &mode, self.registers.y);
                    if opcode == 0xCC {
                        self.registers.pc += 2;
                    } else {
                        self.registers.pc += 1;
                    }
                }
               

                _ => panic!("NO OPCODE MATCH")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_reset_flags() {
        let mut cpu = CPU::new();
        cpu.load_rom(vec![0xa9, 0x05, 0x00]);
        cpu.reset();
        assert!(cpu.status.contains(Flags::I));
        assert!(cpu.status.contains(Flags::U));
    }
    #[test]
    fn test_lda_a9() {
        // let bus = BUS::new();
        let mut cpu = CPU::new();
        cpu.load_rom(vec![0xa9, 0x05, 0x01]);
        cpu.reset();
        cpu.decode();
        assert_eq!(cpu.registers.a, 0x05);
        assert!(cpu.status & Flags::Z == Flags::O);
        assert!(cpu.status & Flags::N == Flags::O);
    }
    #[test]
    fn test_lda_a9_zero() {
        // let bus = BUS::new();
        let mut cpu = CPU::new();
        cpu.load_rom(vec![0xa9, 0x00, 0x01]);
        cpu.reset();
        cpu.decode();
        assert!(cpu.status & Flags::Z == Flags::Z);

    }
    #[test]
    fn test_lda_a9_neg() {
        // let bus = BUS::new();
        let mut cpu = CPU::new();
        cpu.load_rom(vec![0xa9, 0xF0, 0x01]);
        cpu.reset();
        cpu.decode();
        assert!(cpu.status & Flags::N == Flags::N);
    }
    #[test]
    fn test_tax() {
        // let bus = BUS::new();
        let mut cpu = CPU::new();
        cpu.load_rom(vec![0xa9, 0x05, 0xaa, 0x00, 0x01]);
        cpu.reset();
        cpu.decode();
        assert_eq!(cpu.registers.x, cpu.registers.a);
        print!("TAX: {:#04X} {:#04X}\n", cpu.registers.x, cpu.registers.a);
    }
    #[test]
    fn test_load_prg() {
        let mut cpu = CPU::new();
        let testrom = vec![0xa9, 0x05, 0xaa, 0x00, 0x01];
        cpu.load_rom(testrom);
        // cpu.decode();
        for i in 0..0xa {
            println!("{:#04X}", cpu.bus.cpu_mem[0x600 + i as usize]);
        }
    }
    
    #[test]
    fn test_hashmap() {
        let mut cpu = CPU::new();
        let testrom = vec![0xc4, 0x00, 0x01];
        cpu.load_rom(testrom);
        cpu.reset();
        cpu.decode();
    }
    #[test]
    fn test_get_address() {
        let mut cpu = CPU::new();
        let testrom = vec![0xa9, 0x05, 0xad, 0xCC, 0xD1, 0x01];
        cpu.load_rom(testrom);
        cpu.reset();
        cpu.decode();
    }
    #[test]
    fn test_add_carry_immediate() {
        let mut cpu = CPU::new();
        let testrom = vec![0xA9, 0x01, 0x69, 0xFA, 0x00];
        cpu.load_rom(testrom);
        cpu.reset();
        cpu.decode();
        println!("{:?}", cpu.status);
    }
    #[test]
    fn test_add_carry_absolute() {
        let mut cpu = CPU::new();
        let testrom = vec![0xA9, 0xF1, 0x6D, 0x06, 0x06, 0x01, 0xFF, 0x00];
        cpu.load_rom(testrom);
        cpu.reset();
        cpu.decode();
        assert!(cpu.status & Flags::N == Flags::N);
        assert!(cpu.status & Flags::C == Flags::C);
    }
    #[test]
    fn test_add_carry_overflow() {
        let mut cpu = CPU::new();
        // let testrom = vec![0xA9, 0x40, 0x69, 0x40, 0x01, 0x00];
        let testrom = vec![0xA9, 0xC0, 0x69, 0x80, 0x01, 0x00];
        cpu.load_rom(testrom);
        cpu.reset();
        // cpu.status = Flags::C; // Toggle Carry flag for testing operations the carry flag set by previous operation
        cpu.decode();
        assert!(cpu.status & Flags::C == Flags::C);
        assert!(cpu.status & Flags::V == Flags::V);
        println!("{:?}", cpu.status); // Check flags after ADC
    }
}