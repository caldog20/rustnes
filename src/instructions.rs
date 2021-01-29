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
    let addr = cpu.get_address(mode);
    let value = cpu.read_mem(addr);
    set_register_a(cpu, value);
    println!("LDA ADDR: {:#04X} VAL: {:#04X}", addr, value);
}
// LDX
pub fn ldx(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let value = cpu.read_mem(addr);
    cpu.registers.x = value;
    cpu.zero_neg_flags(cpu.registers.x);
}
// LDY
pub fn ldy(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let value = cpu.read_mem(addr);
    cpu.registers.y = value;
    cpu.zero_neg_flags(cpu.registers.y);
}
// ADC
pub fn adc(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let value = cpu.read_mem(addr);
    add_register_a(cpu, value);
}
// SBC
pub fn sbc(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let value = cpu.read_mem(addr);
    add_register_a(cpu, ((value as i8).wrapping_neg().wrapping_sub(1)) as u8);
}
// ASL
pub fn asl(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
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
// ASL ACC
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
// TAY
pub fn tay(cpu: &mut CPU) {
    cpu.registers.y = cpu.registers.a;
    cpu.zero_neg_flags(cpu.registers.y);
}
// TXA
pub fn txa(cpu: &mut CPU) {
    cpu.registers.a = cpu.registers.x;
    cpu.zero_neg_flags(cpu.registers.a);
}
// TSX
pub fn tsx(cpu: &mut CPU) {
    cpu.registers.x = cpu.registers.sp;
    cpu.zero_neg_flags(cpu.registers.x);
}
// TXS
pub fn txs(cpu: &mut CPU) {
    cpu.registers.sp = cpu.registers.x;
    cpu.zero_neg_flags(cpu.registers.sp);
}
// TYA
pub fn tya(cpu: &mut CPU) {
    cpu.registers.a = cpu.registers.y;
    cpu.zero_neg_flags(cpu.registers.a);
}
// AND
pub fn and(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let value = cpu.read_mem(addr);
    let result = cpu.registers.a & value;
    set_register_a(cpu, result);
    cpu.zero_neg_flags(cpu.registers.a)
}
// BITS
pub fn bits(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let value = cpu.read_mem(addr);
    let result = cpu.registers.a & value;
    if result == 0 {
        cpu.update_zero_flag(true);
    } else {
        cpu.update_zero_flag(false);
    }
    cpu.status.set(Flags::N, result & 0b10000000 > 0);
    cpu.status.set(Flags::V, result & 0b01000000 > 0);
}
// CMP Compare universal
pub fn compare(cpu: &mut CPU, mode: &Modes, operand: u8) {
    let addr = cpu.get_address(mode);
    let value = cpu.read_mem(addr);
    if operand >= value {
        cpu.update_carry_flag(true);
    } else {
        cpu.update_carry_flag(false);
    }
    cpu.zero_neg_flags(operand.wrapping_sub(value));
}
// DEC Mem
pub fn dec(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    println!("ADDR {:#04X}", addr);
    let mut value = cpu.read_mem(addr);
    println!("VAL {:#04X}", value);
    value = value.wrapping_sub(1);
    println!("RES VAL {:#04X}", value);
    cpu.write_mem(addr, value);
    cpu.zero_neg_flags(value);
}
// DEC X Register
pub fn dec_x(cpu: &mut CPU) {
        cpu.registers.x = cpu.registers.x.wrapping_sub(1);
        cpu.zero_neg_flags(cpu.registers.x);
}
// DEC Y Register
pub fn dec_y(cpu: &mut CPU) {
    cpu.registers.y = cpu.registers.y.wrapping_sub(1);
    cpu.zero_neg_flags(cpu.registers.y);
}
// CLC
pub fn clc(cpu: &mut CPU) {
    cpu.status.remove(Flags::C);
}
// SEC
pub fn sec(cpu: &mut CPU) {
    cpu.status.insert(Flags::C);
}
// CLI
pub fn cli(cpu: &mut CPU) {
    cpu.status.remove(Flags::I);
}
// SEI
pub fn sei(cpu: &mut CPU) {
    cpu.status.insert(Flags::I);
}
// CLV
pub fn clv(cpu: &mut CPU) {
    cpu.status.remove(Flags::V);
}
//ROL Acc
pub fn rol_a(cpu: &mut CPU) {
    let mut value = cpu.registers.a;
    let new_carry = value >> 7;
    value = value << 1;
    if cpu.status.contains(Flags::C) {
        value = value | 1;
    }
    if new_carry == 1 {
        cpu.update_carry_flag(true);
    } else {
        cpu.update_carry_flag(false);
    }
    set_register_a(cpu, value);
}
// ROL
pub fn rol(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let mut value = cpu.read_mem(addr);
    let new_carry = value >> 7;
    if cpu.status.contains(Flags::C) {
        value = value | 1;
    }
    if new_carry == 1 {
        cpu.update_carry_flag(true);
    } else {
        cpu.update_carry_flag(false);
    }
    cpu.write_mem(addr, value);
    cpu.zero_neg_flags(value);
}
// ROR Acc
pub fn ror_a(cpu: &mut CPU) {
    let mut value = cpu.registers.a;
    let new_carry = value & 1;
    value = value >> 1;
    if cpu.status.contains(Flags::C) {
        value = value | 0b10000000;
    }
    if new_carry == 1 {
        cpu.update_carry_flag(true);
    } else {
        cpu.update_carry_flag(false);
    }
    set_register_a(cpu, value);
}
// ROR
pub fn ror(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let mut value = cpu.read_mem(addr);
    let new_carry = value & 1;
    value = value >> 1;
    if cpu.status.contains(Flags::C) {
        value = value | 0b10000000;
    }
    if new_carry == 1 {
        cpu.update_carry_flag(true);
    } else {
        cpu.update_carry_flag(false);
    }
    cpu.write_mem(addr, value);
    cpu.zero_neg_flags(value);
}
// STA
pub fn sta(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let value = cpu.registers.a;
    cpu.write_mem(addr, value);
}
// STX
pub fn stx(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let value = cpu.registers.x;
    cpu.write_mem(addr, value);
}
// STY
pub fn sty(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let value = cpu.registers.y;
    cpu.write_mem(addr, value);
}
// INC
pub fn inc(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let mut value = cpu.read_mem(addr);
    value = value.wrapping_add(1);
    cpu.write_mem(addr, value);
    cpu.zero_neg_flags(value);
}
// INX
pub fn inx(cpu: &mut CPU) {
    cpu.registers.x = cpu.registers.x.wrapping_add(1);
    cpu.zero_neg_flags(cpu.registers.x);
}
// INY
pub fn iny(cpu: &mut CPU) {
    cpu.registers.y = cpu.registers.y.wrapping_add(1);
    cpu.zero_neg_flags(cpu.registers.y);
}
// LSR Acc
pub fn lsr_a(cpu: &mut CPU) {
    let mut value = cpu.registers.a;
    if value & 1 == 1 {
        cpu.update_carry_flag(true);
    } else {
        cpu.update_carry_flag(false);
    }
    value = value >> 1;
    cpu.registers.a = value;
    cpu.zero_neg_flags(value);
}
// LSR
pub fn lsr(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let mut value = cpu.read_mem(addr);
    if value & 1 == 1 {
        cpu.update_carry_flag(true);
    } else {
        cpu.update_carry_flag(false);
    }
    value = value >> 1;
    cpu.write_mem(addr, value);
    cpu.zero_neg_flags(value);
}
// ORA
pub fn ora(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let value = cpu.read_mem(addr);
    set_register_a(cpu, value | cpu.registers.a);
}
//EOR
pub fn eor(cpu: &mut CPU, mode: &Modes) {
    let addr = cpu.get_address(mode);
    let value = cpu.read_mem(addr);
    set_register_a(cpu, value ^ cpu.registers.a);
}
// BRANCH TEST
pub fn branch(cpu: &mut CPU, flag: bool) {
    if flag {
        let jmp: i8 = cpu.read_mem(cpu.registers.pc) as i8;
        let jmp_to = cpu.registers.pc
            .wrapping_add(1)
            .wrapping_add(jmp as u16);
        cpu.registers.pc = jmp_to;
    }
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
    #[test]
    fn test_cmp() {
        let mut cpu = CPU::new();
        let testrom = vec![0x02, 0x06, 0x05];
        cpu.load_rom(testrom);
        cpu.registers.pc = 0x600;
        compare(&mut cpu, &Modes::Absolute, 0x04);
        println!("{:?}", cpu.status);
    }
    #[test]
    fn test_dec() {
        let mut cpu = CPU::new();
        let testrom = vec![0x02, 0x06, 0x05];
        cpu.load_rom(testrom);
        cpu.registers.pc = 0x600;
        dec(&mut cpu, &Modes::Absolute);
        println!("{:#04X}", cpu.bus.cpu_mem[0x602]);
    }
    #[test]
    fn test_and() {
        let mut cpu = CPU::new();
        let testrom = vec![0x02, 0x06, 0xA1];
        cpu.load_rom(testrom);
        cpu.registers.a = 0x23;
        cpu.registers.pc = 0x600;
        and(&mut cpu, &Modes::Absolute);
        println!("{:#04X}", cpu.registers.a);
    }
}