use crate::cpu::MEM;

pub struct BUS {
    cpu_mem: [u8; 0x800],
    // prg_mem: [u8; 0x8000],
    // ppu_ram: [u8, ]
}
impl BUS {
    pub fn new() -> Self {
        BUS {
            cpu_mem: [0; 0x800]
        }  
    }
}
impl MEM for BUS {

}