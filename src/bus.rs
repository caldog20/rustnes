use crate::cpu::MEM;


pub struct BUS {
    pub cpu_mem: [u8; 0x800],
    pub prg_mem: [u8; 0x8000],
    // ppu_ram: [u8, ]
}
impl BUS {
    pub fn new() -> Self {
        BUS {
            cpu_mem: [0; 0x800],
            prg_mem: [0; 0x8000]
        }
        }  
    pub fn read_prg_mem(&self, mut addr:u16) -> u8 {
        addr -= 0x8000;
        self.prg_mem[addr as usize]
    }
}
impl MEM for BUS {
    fn read_mem(&self, addr: u16) -> u8 {
        match addr {
        0x0000 ..= 0x1FFF => {
            let masked_addr = addr & 0b111_11111111;
            self.cpu_mem[masked_addr as usize]
        }
        0x8000 ..= 0xFFFF => self.read_prg_mem(addr),
        _ => panic!("READ MEM: ADDR NO MATCHED")
        }
    }
    fn write_mem(&mut self, addr: u16, data: u8) {
        match addr {
        0x0000 ..= 0x1FFF => {
            let masked_addr = addr & 0b111_11111111;
            self.cpu_mem[masked_addr as usize] = data;
        }
        _ => panic!("WRITE MEM: ADDR NOT MATCHED")
        }
    }
}