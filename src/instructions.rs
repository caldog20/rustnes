use crate::cpu::*;


// pub fn lda(cpu: &mut CPU, value: u8) {
//     cpu.registers.a = value;
//     cpu.zero_neg_flags(cpu.registers.a);
//     cpu.registers.pc += 1;
// }
pub fn tax(cpu: &mut CPU) {
    cpu.registers.x = cpu.registers.a;
    cpu.zero_neg_flags(cpu.registers.x);
    cpu.registers.pc += 1;
}
pub fn lda(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(&mode);
    let value = cpu.read_mem(addr);
    cpu.registers.a = value;
    cpu.zero_neg_flags(cpu.registers.a);
    println!("LDA ADDR: {:#04X} VAL: {:#04X}", addr, value);
}