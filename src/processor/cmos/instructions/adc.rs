use std::ops::Add;
use crate::memory::address::{Address, ZeroPageAddress};
use crate::memory::Memory;
use crate::processor::cmos::CmosProcessor;
use crate::processor::{ExecutionMetrics, Register8, Value};
use crate::processor::status::FLAG_CARRY;

impl<'m, M: Memory> CmosProcessor<'m, M> {
    pub(crate) fn adc_internal(&mut self, value: &Value) {
        let carry = self.status.get_bit_u8(FLAG_CARRY);
        let sum = (self.accumulator as u16) + (carry as u16) + (*value as u16);

        self.accumulator = (0xff & sum) as Register8;

        self.status.clear_bit(FLAG_CARRY);

        let carry_flag = (sum >> 8 & 1) == 1;
        self.status.set_bit(FLAG_CARRY, carry_flag);
    }

    // opc 69
    pub(crate) fn adc_immediate(&mut self, value: &Value) -> ExecutionMetrics {
        self.adc_internal(value);
        ExecutionMetrics::new(2, 2)
    }

    // opc 65
    pub(crate) fn adc_zeropage(&mut self, zp_address: &ZeroPageAddress) -> ExecutionMetrics {
        let value = self.address_zeropage(zp_address);
        self.adc_internal(&value);
        ExecutionMetrics::new(2, 3)
    }

    // opc 75
    pub(crate) fn adc_zeropage_x(&mut self, zp_address: &ZeroPageAddress) -> ExecutionMetrics {
        let value = self.address_zeropage_x(zp_address);
        self.adc_internal(&value);
        ExecutionMetrics::new(2, 4)
    }

    pub(crate) fn adc_absolute(&mut self, address: &Address) -> ExecutionMetrics {
        let value = self.address_absolute(address);
        self.adc_internal(&value);
        ExecutionMetrics::new(3, 4)
    }


    pub(crate) fn adc_absolute_x(&mut self, address: &Address) -> ExecutionMetrics {
        let (value, page_crossed) = self.address_absolute_x(address);
        self.adc_internal(&value);
        ExecutionMetrics::new(3, 4 + (page_crossed as u8))
    }

    pub(crate) fn adc_absolute_y(&mut self, address: &Address) -> ExecutionMetrics {
        let (value, page_crossed) = self.address_absolute_y(address);
        self.adc_internal(&value);
        ExecutionMetrics::new(3, 4 + (page_crossed as u8))
    }

    pub(crate) fn adc_preindexed_indirect_x(
        &mut self,
        zp_address: &ZeroPageAddress,
    ) -> ExecutionMetrics {
       let value = self.address_preindexed_indirect_x(zp_address);
        self.adc_internal(&value);
        ExecutionMetrics::new(2, 6)
    }

    pub(crate) fn adc_postindexed_indirect_y(
        &mut self,
        zp_address: &ZeroPageAddress,
    ) -> ExecutionMetrics {
        let (value, page_crossed) = self.address_postindexed_indirect_y(zp_address);
        self.adc_internal(&value);
        ExecutionMetrics::new(2, 5 + (page_crossed as u8))
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address::{Address, AddressMode, ZeroPageAddress};
    use crate::processor::instructions::Instruction;
    use crate::memory::Memory;
    use crate::memory::VecMemory;
    use crate::processor::cmos::CmosProcessor;
    use crate::processor::status::FLAG_CARRY;

    #[test]
    fn test_adc_immediate() {
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

    #[test]
    fn test_adc_zeropage() {
        let zp_address = ZeroPageAddress(32);
        let address = zp_address.upgrade();
        let mut memory = VecMemory::default();
        memory.write(&address, &5);
        let mut processor = CmosProcessor::with_memory(&mut memory);

        processor.execute(&Instruction::ADC, &AddressMode::ZeroPage(zp_address));

        assert_eq!(processor.accumulator, 5);
    }

    #[test]
    fn test_adc_zeropage_x() {
        let zp_address = ZeroPageAddress(32);
        let address = zp_address.upgrade();

        let mut memory = VecMemory::default();
        memory.write(&address, &5);

        let mut processor = CmosProcessor::with_memory(&mut memory);

        // 30 + 2 = 32 so we should read 5 at address 32
        processor.x = 2;
        let zp_address = ZeroPageAddress(30);

        processor.execute(&Instruction::ADC, &AddressMode::ZeroPageX(zp_address));

        assert_eq!(processor.accumulator, 5);
    }

    #[test]
    fn test_adc_absolute() {
        let address = Address(0xcabd);

        let mut memory = VecMemory::default();
        memory.write(&address, &5);

        let mut processor = CmosProcessor::with_memory(&mut memory);
        processor.execute(&Instruction::ADC, &AddressMode::Absolute(address));

        assert_eq!(processor.accumulator, 5);
    }

    #[test]
    fn test_adc_absolute_x() {
        let address = Address(0xcabd);

        let mut memory = VecMemory::default();
        memory.write(&address, &5);

        let mut processor = CmosProcessor::with_memory(&mut memory);

        // cabb + 2 = cabd so we should read 5 at address 32
        processor.x = 2;
        let address = Address(0xcabb);

        processor.execute(&Instruction::ADC, &AddressMode::AbsoluteX(address));

        assert_eq!(processor.accumulator, 5);
    }

    #[test]
    fn test_adc_absolute_y() {
        let address = Address(0xcabd);

        let mut memory = VecMemory::default();
        memory.write(&address, &5);

        let mut processor = CmosProcessor::with_memory(&mut memory);

        // cabb + 2 = cabd so we should read 5 at address 32
        processor.y = 2;
        let address = Address(0xcabb);

        processor.execute(&Instruction::ADC, &AddressMode::AbsoluteY(address));

        assert_eq!(processor.accumulator, 5);
    }

    #[test]
    fn test_adc_preindexed_indirect_x() {
        let mut memory = VecMemory::default();
        memory.write(&Address(0x0010), &0xbb);
        memory.write(&Address(0x0011), &0xca);
        memory.write(&Address(0xcabb), &0x32);

        let mut processor = CmosProcessor::with_memory(&mut memory);
        processor.x = 0x08;

        // 0x00 = 8, x = 8, 8+8=0x10, 0x10,11 -> 0xbb,0xca -- 0xcabb -> 0x32
        processor.execute(
            &Instruction::ADC,
            &AddressMode::PreIndexedIndirectX(ZeroPageAddress(0x08)),
        );

        assert_eq!(processor.accumulator, 0x32);
    }

    #[test]
    fn test_adc_postindexed_indirect_y() {
        let mut memory = VecMemory::default();

        memory.write(&Address(0x0010), &(0xbb - 0x20));
        memory.write(&Address(0x0011), &0xca);
        memory.write(&Address(0xcabb), &0x32);

        let mut processor = CmosProcessor::with_memory(&mut memory);
        processor.y = 0x20;

        // 0x00 = 8, x = 8, 8+8=0x10, 0x10,11 -> 0xbb,0xca -- 0xcabb -> 0x32
        processor.execute(
            &Instruction::ADC,
            &AddressMode::PostIndexedIndirectY(ZeroPageAddress(0x10)),
        );

        assert_eq!(processor.accumulator, 0x32);
    }
}
