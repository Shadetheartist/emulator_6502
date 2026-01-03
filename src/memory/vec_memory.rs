use crate::memory::address::Address;
use crate::memory::Memory;
use crate::processor::Value;

pub struct VecMemory(Vec<u8>);

impl Default for VecMemory {
    fn default() -> Self {
        Self(vec![0; 0x10000])
    }
}


impl Memory for VecMemory {
    fn read(&self, address: &Address) -> Value {
        self.0[address.0 as usize]
    }
    fn write(&mut self, address: &Address, value: &Value) {
        self.0[address.0 as usize] = *value
    }
}