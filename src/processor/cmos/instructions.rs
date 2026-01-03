use crate::memory::Memory;
use crate::processor::{Register8, Value};
use crate::processor::cmos::CmosProcessor;
use crate::processor::status::FLAG_CARRY;


impl<'m, M: Memory> CmosProcessor<'m, M> {
    pub(crate) fn execute_adc(&mut self, value: &Value) {
        let carry = self.status.get_bit_u8(FLAG_CARRY);
        let sum = (self.accumulator as u16) + (carry as u16) + (*value as u16);

        self.accumulator = (0xff & sum) as Register8;

        self.status.clear_bit(FLAG_CARRY);

        let carry_flag = (sum >> 8 & 1) == 1;
        self.status.set_bit(FLAG_CARRY, carry_flag);
    }

    pub(crate) fn execute_and(&mut self, value: &Value){
        self.accumulator &= *value
    }
}


#[cfg(test)]
mod test {
    use crate::memory::address::AddressMode;
    use crate::memory::vec_memory::VecMemory;
    use crate::processor::cmos::CmosProcessor;
    use crate::processor::Instruction;
    use crate::processor::status::FLAG_CARRY;

    #[test]
    fn test_adc() {
        let mut memory = VecMemory::default();
        let mut processor = CmosProcessor::with_memory(&mut memory);

        // add small value
        processor.execute(&Instruction::ADC, &AddressMode::Immediate(5));
        assert_eq!(processor.accumulator, 5);
        assert_eq!(processor.status.get_bit(FLAG_CARRY), false);

        // bring right to the edge of overflowing
        processor.execute(&Instruction::ADC, &AddressMode::Immediate(250));
        assert_eq!(processor.accumulator, 255);
        assert_eq!(processor.status.get_bit(FLAG_CARRY), false);

        // make sure carry is set after overflow, and acc is modulo 256
        processor.execute(&Instruction::ADC, &AddressMode::Immediate(2));
        assert_eq!(processor.accumulator, 1);
        assert_eq!(processor.status.get_bit(FLAG_CARRY), true);

        // make sure carry is added when set
        processor.execute(&Instruction::ADC, &AddressMode::Immediate(1));
        assert_eq!(processor.accumulator, 3);
        assert_eq!(processor.status.get_bit(FLAG_CARRY), false);
    }
}