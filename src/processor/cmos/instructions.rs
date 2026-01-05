use crate::memory::address::AddressMode;
use crate::memory::Memory;
use crate::processor::cmos::CmosProcessor;
use crate::processor::status::{FLAG_CARRY, FLAG_ZERO};
use crate::processor::Register8;

impl<'m, M: Memory> CmosProcessor<'m, M> {
    pub(crate) fn execute_adc(&mut self, address_mode: &AddressMode) -> u8 {
        let (value, additional_cycles) = self.read_address(address_mode);

        let carry = self.status.get_bit_u8(FLAG_CARRY);
        let sum = (self.accumulator as u16) + (carry as u16) + (value as u16);

        self.accumulator = (0xff & sum) as Register8;

        self.status.clear_bit(FLAG_CARRY);

        let carry_flag = (sum >> 8 & 1) == 1;
        self.status.set_bit(FLAG_CARRY, carry_flag);

        self.status.clear_bit(FLAG_ZERO);
        self.status.set_bit(FLAG_ZERO, self.accumulator == 0);

        additional_cycles
    }

    pub(crate) fn execute_and(&mut self, address_mode: &AddressMode) -> u8 {
        let (value, additional_cycles) = self.read_address(address_mode);

        self.accumulator &= value;

        additional_cycles
    }

    pub(crate) fn execute_asl(&mut self, address_mode: &AddressMode) -> u8 {
        match address_mode {
            AddressMode::Implied => {
                self.accumulator = self.accumulator << 1;
                0
            }
            _ => {
                let (address, additional_cycles) = self
                    .translate_address(address_mode)
                    .expect("addressing mode should return Some");

                let mut value = self.memory.read(&address);

                value = value << 1;

                self.memory.write(&address, &value);

                additional_cycles
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address::AddressMode;
    use crate::memory::vec_memory::VecMemory;
    use crate::processor::cmos::CmosProcessor;
    use crate::processor::status::{FLAG_CARRY, FLAG_OVERFLOW, FLAG_ZERO};
    use crate::processor::Instruction;

    #[test]
    fn test_adc() {
        let mut memory = VecMemory::default();
        let mut processor = CmosProcessor::with_memory(&mut memory);

        // add small value
        processor.execute(&Instruction::ADC, &AddressMode::Immediate(5));
        assert_eq!(processor.accumulator, 5);
        assert_eq!(processor.status.get_bit(FLAG_CARRY), false);
        assert_eq!(processor.status.get_bit(FLAG_ZERO), false);

        // bring right to the edge of overflowing
        processor.execute(&Instruction::ADC, &AddressMode::Immediate(250));
        assert_eq!(processor.accumulator, 255);
        assert_eq!(processor.status.get_bit(FLAG_CARRY), false);
        assert_eq!(processor.status.get_bit(FLAG_OVERFLOW), false);
        assert_eq!(processor.status.get_bit(FLAG_ZERO), false);

        // make sure carry is set after overflow, and acc is modulo 256
        processor.execute(&Instruction::ADC, &AddressMode::Immediate(1));
        assert_eq!(processor.accumulator, 0);
        assert_eq!(processor.status.get_bit(FLAG_CARRY), true);
        assert_eq!(processor.status.get_bit(FLAG_ZERO), true);

        // make sure carry is added when set
        processor.execute(&Instruction::ADC, &AddressMode::Immediate(1));
        assert_eq!(processor.accumulator, 2);
        assert_eq!(processor.status.get_bit(FLAG_CARRY), false);
        assert_eq!(processor.status.get_bit(FLAG_ZERO), false);
    }

    #[test]
    fn test_and() {
        let mut memory = VecMemory::default();
        let mut processor = CmosProcessor::with_memory(&mut memory);
        processor.accumulator = 0b1100;

        processor.execute(&Instruction::AND, &AddressMode::Immediate(0b1010));
        assert_eq!(processor.accumulator, 0b1000);

    }
}
