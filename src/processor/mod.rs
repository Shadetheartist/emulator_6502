pub mod cmos;
pub mod instructions;
pub mod status;

pub type Value = u8;
type Register16 = u16;
type Register8 = u8;

struct ExecutionMetrics {
    bytes: u8,
    cycles: u8,
}

impl ExecutionMetrics {
    fn new(bytes: u8, cycles: u8) -> Self {
        Self { bytes, cycles }
    }
}