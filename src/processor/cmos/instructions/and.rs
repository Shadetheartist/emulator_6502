use crate::memory::Memory;
use crate::processor::cmos::CmosProcessor;
use crate::processor::{ExecutionMetrics, Value};

impl<'m, M: Memory> CmosProcessor<'m, M> {
    fn and_internal(&mut self, value: &Value){
        self.accumulator &= *value
    }

    pub(crate) fn and_immediate(&mut self, value: &Value) -> ExecutionMetrics {
        self.and_internal(value);

        ExecutionMetrics::new(2, 2)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address::{Address, AddressMode};
    use crate::memory::VecMemory;
    use crate::processor::cmos::CmosProcessor;
    use crate::processor::instructions::Instruction;

    #[test]
    fn test_and_immediate(){
        let mut memory = VecMemory::default();
        let mut processor = CmosProcessor::with_memory(&mut memory);
        processor.accumulator = 0b1100;

        processor.execute(&Instruction::AND, &AddressMode::Immediate(0b1010));

        assert_eq!(processor.accumulator, 0b1000u8);
    }
}