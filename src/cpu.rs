
#[allow(dead_code)]
const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xFD;
bitflags! {
    pub struct Flags: u8{
    const C = 0b00000001;
    const Z = 0b00000010;
    const I = 0b00000100;
    const D = 0b00001000;
    const B = 0b00010000;
    const U = 0b00100100;
    const V = 0b01000000;
    const N = 0b10000000;
    }
}

pub struct CPU {
    pub pc: u16,
    pub sp: u8,
    pub acc: u8,
    pub status: Flags,
    pub vx: u8,
    pub vy: u8,
    pub mem: [u8; 0xFFFF]
}
pub enum Modes {
    Immediate,
    ZeroPage,
    ZeroPX,
    ZeroPY,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            sp: STACK_RESET,
            acc: 0,
            status: Flags::U,
            vx: 0,
            vy: 0,
            mem: [0; 0xFFFF]
        }
    }
    pub fn mem_read(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }
    pub fn mem_write(&mut self, addr: u16, value: u8) {
        self.mem[addr as usize] = value;
    }
    pub fn get_opaddr(&self, mode: &Modes) -> u16 {
        match mode {
            Modes::Immediate => self.pc,
            _ => panic!()
            // Modes::ZeroPage => panic!(),
            // Modes::ZeroPX => panic!(),
            // Modes::ZeroPY,
            // Modes::Absolute,
            // Modes::Absolute_X,
            // Modes::Absolute_Y,
            // Modes::Indirect_X,
            // Modes::Indirect_Y,
            // Modes::NoneAddressing
        }
    }
    pub fn decode(&mut self, program: Vec<u8>) {
        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;

            match opcode {
                0x00 => return,
                0xA9 => {
                    // let param = program[self.pc as usize];
                    self.pc += 1;
                    self.lda(&Modes::Immediate)
                }
                _ => println!("Opcode NOT matched!")
            }

        }
    }
    pub fn zero_negative(&mut self, result: u8) {
        if result == 0 {
            self.status.insert(Flags::Z);
        }
        else {
            self.status.remove(Flags::Z);
        }
        if result & 0b10000000 != 0 {
            self.status.insert(Flags::N);
        }
        else {
            self.status.remove(Flags::N);
        }
    }
    pub fn lda(&mut self, mode: &Modes) {
        let addr = self.get_opaddr(mode);
        println!("{:#04x}", addr);
        let value = self.mem_read(addr);
        // println!("{:#05X}", value);
        self.acc = value;
        self.zero_negative(self.acc);
    }
}

pub fn testcpu() {
    let mut cpu = CPU::new();
    println!("{:#b}", cpu.status);
    cpu.mem_write(0x10, 0x55);
    cpu.decode(vec![0xa9, 0x10, 0x00]);
    cpu.mem_write(0x11, 0x00);
    cpu.decode(vec![0xa9, 0x11, 0x00]);
    println!("{:#b}", cpu.status);
    // let res = cpu.status & 0b00000010;
    // println!("{:?}", res);
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_lda_a9() {
//         let mut cpu = CPU::new();
//         cpu.mem_write(0x10, 0x55);
//         cpu.decode(vec![0xa9, 0x00, 0x00]);
//         println!("{:?}", cpu.status);
//         let res = cpu.status & Flags::Z;
//         println!("{:?}", res);
//     }
// }