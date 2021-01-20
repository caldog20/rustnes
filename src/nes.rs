#![allow(dead_code)]
use crate::bus::*;
use crate::cpu::*;

pub struct NES {
    cpu: CPU,
    bus: BUS,
}

impl NES {
    cpu: CPU::new(),
    bus: bus::new()
}
