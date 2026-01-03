pub use vec_memory::VecMemory;
use address::Address;
use crate::processor::Value;

mod vec_memory;
pub mod address;

pub trait Memory {
    fn read(&self, address: &Address) -> Value;
    fn write(&mut self, address: &Address, value: &Value);
}
