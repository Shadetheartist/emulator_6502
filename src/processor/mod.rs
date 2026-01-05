pub mod cmos;
mod instructions;
mod status;

pub use instructions::Instruction;

pub type Value = u8;
type Register16 = u16;
type Register8 = u8;

pub struct ExecutionMetrics {
    pub op_code: u8,
    pub bytes: u8,
    pub cycles: u8,
}

impl ExecutionMetrics {
    pub fn new(op_code: u8, bytes: u8, cycles: u8) -> Self {
        Self {
            op_code,
            bytes,
            cycles,
        }
    }
}

#[inline(always)]
fn get_bit<T: num_traits::PrimInt>(value: T, bit: usize) -> bool
{
    (value >> bit) & T::one() == T::one()
}