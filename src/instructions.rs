use crate::cpu::CPU;


pub fn lda(cpu: &mut CPU, value: u8) {
    cpu.registers.a = value;
    cpu.zero_neg_flags(cpu.registers.a);
    cpu.registers.pc += 1;
}
pub fn tax(cpu: &mut CPU) {
    cpu.registers.x = cpu.registers.a;
    cpu.zero_neg_flags(cpu.registers.x);
    cpu.registers.pc += 1;
}