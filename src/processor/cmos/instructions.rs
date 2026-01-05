use crate::memory::address::AddressMode;
use crate::memory::Memory;
use crate::processor::cmos::CmosProcessor;
use crate::processor::status::{FLAG_CARRY, FLAG_NEGATIVE, FLAG_OVERFLOW, FLAG_ZERO};
use crate::processor::{get_bit, Register8};

impl<'m, M: Memory> CmosProcessor<'m, M> {

    #[inline(always)]
    fn set_zero_flag(&mut self){
        self.status.clear_bit(FLAG_ZERO);
        self.status.set_bit(FLAG_ZERO, self.accumulator == 0);
    }

    #[inline(always)]
    fn set_negative_flag(&mut self){
        let negative_flag = get_bit(self.accumulator, 7);
        self.status.clear_bit(FLAG_NEGATIVE);
        self.status.set_bit(FLAG_NEGATIVE, negative_flag);
    }

    // implemented as instructed from
    // https://www.xjavascript.com/blog/6502-emulation-proper-way-to-implement-adc-and-sbc
    pub(crate) fn execute_adc(&mut self, address_mode: &AddressMode) -> u8 {
        let (value, additional_cycles) = self.read_address(address_mode);

        let carry = self.status.get_bit(FLAG_CARRY);

        let overflow_value = value & 0b01111111;
        let overflow_acc = self.accumulator & 0b01111111;
        let overflow_carry_in = overflow_acc + overflow_value + carry as u8;

        let sum = (self.accumulator as u16) + (value as u16) + (carry as u16);

        self.accumulator = (0xff & sum) as Register8;

        self.status.clear_bit(FLAG_CARRY);

        let carry_flag = get_bit(sum, 8);
        self.status.set_bit(FLAG_CARRY, carry_flag);

        let overflow_carry_out = carry_flag as u8;
        let overflow = overflow_carry_in ^ overflow_carry_out;
        self.status.clear_bit(FLAG_OVERFLOW);
        self.status.set_bit(FLAG_OVERFLOW, overflow == 1);

        self.set_zero_flag();
        self.set_negative_flag();

        additional_cycles
    }

    pub(crate) fn execute_and(&mut self, address_mode: &AddressMode) -> u8 {
        let (value, additional_cycles) = self.read_address(address_mode);

        self.accumulator &= value;

        self.set_zero_flag();
        self.set_negative_flag();

        additional_cycles
    }

    pub(crate) fn execute_asl(&mut self, address_mode: &AddressMode) -> u8 {
        match address_mode {
            AddressMode::Implied => {
                self.accumulator = self.accumulator << 1;

                self.set_zero_flag();
                self.set_negative_flag();

                0
            }
            _ => {
                let (address, additional_cycles) = self
                    .translate_address(address_mode)
                    .expect("addressing mode should return Some");

                let mut value = self.memory.read(&address);

                value = value << 1;

                self.memory.write(&address, &value);

                self.set_zero_flag();
                self.set_negative_flag();

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
    use crate::processor::status::{FLAG_CARRY, FLAG_NEGATIVE, FLAG_OVERFLOW, FLAG_ZERO};
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

        // reset processor & memory
        let mut memory = VecMemory::default();
        let mut processor = CmosProcessor::with_memory(&mut memory);

        processor.execute(&Instruction::ADC, &AddressMode::Immediate(64));
        assert_eq!(processor.accumulator, 64);
        assert_eq!(processor.status.get_bit(FLAG_CARRY), false);
        assert_eq!(processor.status.get_bit(FLAG_ZERO), false);
        assert_eq!(processor.status.get_bit(FLAG_OVERFLOW), false);
        assert_eq!(processor.status.get_bit(FLAG_NEGATIVE), false);

        // test negative value (when thinking in twos compliment signed way)
        processor.execute(&Instruction::ADC, &AddressMode::Immediate(64));
        assert_eq!(processor.accumulator, 128);
        assert_eq!(processor.status.get_bit(FLAG_CARRY), false);
        assert_eq!(processor.status.get_bit(FLAG_ZERO), false);
        assert_eq!(processor.status.get_bit(FLAG_OVERFLOW), false);
        assert_eq!(processor.status.get_bit(FLAG_NEGATIVE), true);

        // overflow back to zero
        processor.execute(&Instruction::ADC, &AddressMode::Immediate(128));
        assert_eq!(processor.accumulator, 0);
        assert_eq!(processor.status.get_bit(FLAG_CARRY), true);
        assert_eq!(processor.status.get_bit(FLAG_ZERO), true);
        assert_eq!(processor.status.get_bit(FLAG_OVERFLOW), true);
        assert_eq!(processor.status.get_bit(FLAG_NEGATIVE), false);
    }

    #[test]
    fn test_and() {
        let mut memory = VecMemory::default();
        let mut processor = CmosProcessor::with_memory(&mut memory);
        processor.accumulator = 0b1100;

        processor.execute(&Instruction::AND, &AddressMode::Immediate(0b1010));
        assert_eq!(processor.accumulator, 0b1000);
        assert_eq!(processor.status.get_bit(FLAG_ZERO), false);
        assert_eq!(processor.status.get_bit(FLAG_NEGATIVE), false);

    }

    #[test]
    fn test_asl() {
        let mut memory = VecMemory::default();
        let mut processor = CmosProcessor::with_memory(&mut memory);
        processor.accumulator = 0b00110000;

        processor.execute(&Instruction::ASL, &AddressMode::Implied);
        assert_eq!(processor.accumulator, 0b01100000);
        assert_eq!(processor.status.get_bit(FLAG_ZERO), false);
        assert_eq!(processor.status.get_bit(FLAG_NEGATIVE), false);

        processor.execute(&Instruction::ASL, &AddressMode::Implied);
        assert_eq!(processor.accumulator, 0b11000000);

        assert_eq!(processor.status.get_bit(FLAG_ZERO), false);
        assert_eq!(processor.status.get_bit(FLAG_NEGATIVE), true);

        processor.execute(&Instruction::ASL, &AddressMode::Implied);
        processor.execute(&Instruction::ASL, &AddressMode::Implied);

        assert_eq!(processor.accumulator, 0b00000000);
        assert_eq!(processor.status.get_bit(FLAG_ZERO), true);
        assert_eq!(processor.status.get_bit(FLAG_NEGATIVE), false);
    }
}
