use address::Address;
use crate::processor::Value;

pub mod vec_memory;
pub mod address;

pub trait Memory {
    fn read(&self, address: &Address) -> Value;
    fn write(&mut self, address: &Address, value: &Value);
}
