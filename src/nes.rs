#![allow(dead_code)]
use crate::cpu::*;

pub struct NES {
    pub cpu: CPU,
}

impl NES {
    pub fn new() -> Self {
        NES { 
            cpu: CPU::new(),
        }
    }
}

