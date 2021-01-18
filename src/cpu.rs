
#![allow(dead_code)]
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
pub struct CPU {
    pub pc: u16,
    pub sp: u8,
    pub acc: u8,
    pub status: Flags,
    pub ix: u8,
    pub iy: u8,
    pub mem: [u8; 0xFFFF]
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            sp: 0,
            acc: 0,
            status: Flags::U,
            ix: 0,
            iy: 0,
            mem: [0; 0xFFFF]
        }
    }
    pub fn decode(&mut self, program: Vec<u8>) {
        self.pc = 0;
        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;

            match opcode {
                0x00 => return,
                0x01 => print!("0x01"),
                0xA9 => {
                    let param = program[self.pc as usize];
                    // self.pc += 1;
                    self.acc = param;
                    if self.acc == 0 {
                        self.status.insert(Flags::Z)
                    } else {
                        self.status.remove(Flags::Z)
                    }
                    if self.acc & 0b1000_0000 != 0 {
                        self.status.insert(Flags::N)
                    } else {
                        self.status.remove(Flags::N)
                    }
                }
                _ => print!("Opcode not found.")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_lda_a9() {
        let mut cpu = CPU::new();
        cpu.decode(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.acc, 0x05);
        assert!(cpu.status & Flags::Z == Flags::U);
        assert!(cpu.status & Flags::N == Flags::U);
        println!("{:#b}", cpu.status);
    }
    fn test_lda_a9_zero() {
        let mut cpu = CPU::new();
        cpu.decode
    }
}