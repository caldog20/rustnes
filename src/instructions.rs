// TODO: Add remaining instructions

use crate::cpu::*;


// Gen Purpose Set Acc Register
pub fn set_register_a(cpu: &mut CPU, value: u8) {
    cpu.registers.a = value;
    cpu.zero_neg_flags(cpu.registers.a);
}
// Gen purpose Acc Register ADD
pub fn add_register_a(cpu: &mut CPU, value: u8) { // Possibly clean this up somehow
    let carry = (if cpu.status.contains(Flags::C) {1} else {0}) as u16;
    let base = cpu.registers.a as u16; // Cast Acc register value as u16    
    // println!("base: {}", base);
    let sum = base + carry + value as u16; // Add Acc value to value in memory plus carry(1) if carry was previously set
    // println!("Sum: {}", sum);
    let result = sum as u8; // Cast result as u8 and remove carry bits
    // println!("result: {}", result);
    let carry = sum >> 8; // Shift carry bit to first bit of byte to check if 0 or 1
    if carry == 1 {
        cpu.update_carry_flag(true); // Set carry again if 1
    } else {
        cpu.update_carry_flag(false); // Clear carry if 0
    }
    let overflow = (base as u8 ^ result) & (value ^ result) & 0x80; //
    if overflow != 0 {
        cpu.update_overflow_flag(true);
    } else {
        cpu.update_overflow_flag(false);
    }
    set_register_a(cpu, result);
}
// LDA 
pub fn lda(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(&mode);
    let value = cpu.read_mem(addr);
    set_register_a(cpu, value);
    println!("LDA ADDR: {:#04X} VAL: {:#04X}", addr, value);
}
// ADC
pub fn adc(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(&mode);
    let value = cpu.read_mem(addr);
    add_register_a(cpu, value);
}
// SBC
pub fn sbc(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(&mode);
    let value = cpu.read_mem(addr);
    add_register_a(cpu, ((value as i8).wrapping_neg().wrapping_sub(1)) as u8);
}
//
pub fn asl(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(&mode);
    let mut value = cpu.read_mem(addr);
    if value >> 7 == 1 {
        cpu.update_carry_flag(true);
    } else {
        cpu.update_carry_flag(false);
    }
    value = value << 1;
    cpu.write_mem(addr, value);
    cpu.zero_neg_flags(value);
}
pub fn asl_acc(cpu: &mut CPU) {
    let mut value = cpu.registers.a;
    if value >> 7 == 1 {
        cpu.update_carry_flag(true);
    } else {
        cpu.update_carry_flag(false);
    }
    value = value << 1;
    set_register_a(cpu, value);
}
// TAX
pub fn tax(cpu: &mut CPU) {
    cpu.registers.x = cpu.registers.a;
    cpu.zero_neg_flags(cpu.registers.x);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_asl_acc() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x80;
        asl_acc(&mut cpu);
        println!("{:#04X?}", cpu.registers.a);
        println!("{:?}", cpu.status);
    }
    fn test_sbc() {
        // let mut cpu = CPU::new();

    }
}