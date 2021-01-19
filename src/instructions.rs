use crate::cpu::CPU;


pub fn lda(cpu: &mut CPU, value: u8) {
    cpu.acc = value;
    cpu.zero_neg_flags(cpu.acc);
    cpu.pc += 1;
}
pub fn tax(cpu: &mut CPU) {
    cpu.ix = cpu.acc;
    cpu.zero_neg_flags(cpu.ix);
    cpu.pc += 1;

}