use std::ops::Add;
use crate::memory::address::{Address, AddressMode, ZeroPageAddress};
use crate::memory::Memory;
use crate::processor::cmos::CmosProcessor;
use crate::processor::Value;

impl<'m, M: Memory> CmosProcessor<'m, M> {

    pub(crate) fn translate_address(&self, address_mode: &AddressMode) -> (Value, u8) {
        match address_mode {
            AddressMode::Immediate(value) => (*value, 0),
            AddressMode::ZeroPage(zp_address) => (self.address_zeropage(zp_address), 0),
            AddressMode::ZeroPageX(zp_address) => (self.address_zeropage_x(zp_address), 0),
            AddressMode::ZeroPageY(_) => unimplemented!(),
            AddressMode::Absolute(address) => (self.address_absolute(address), 0),
            AddressMode::AbsoluteX(address) => self.address_absolute_x(address),
            AddressMode::AbsoluteY(address) => self.address_absolute_y(address),
            AddressMode::Indirect(address) => (self.address_absolute(address), 0),
            AddressMode::PreIndexedIndirectX(zp_address) => (self.address_preindexed_indirect_x(zp_address), 0),
            AddressMode::PostIndexedIndirectY(zp_address) => self.address_postindexed_indirect_y(zp_address),
            AddressMode::Relative(_) => unimplemented!(),
        }
    }

    fn address_zeropage(&self, zp_address: &ZeroPageAddress) -> Value {
        let address = zp_address.upgrade();
        self.memory.read(&address)
    }

    fn address_zeropage_x(&self, zp_address: &ZeroPageAddress) -> Value {
        let zp_address = zp_address.wrapping_add(self.x);
        let address = zp_address.upgrade();
        self.memory.read(&address)
    }

    fn address_absolute(&self, address: &Address) -> Value {
        self.memory.read(address)
    }

    fn address_absolute_x(&self, address: &Address) -> (Value, u8) {
        let (address, page_crossed) = address.add_check_page_cross(self.x);
        (self.memory.read(&address), page_crossed as u8)
    }

    fn address_absolute_y(&self, address: &Address) -> (Value, u8) {
        let (address, page_crossed) = address.add_check_page_cross(self.y);
        (self.memory.read(&address), page_crossed as u8)
    }

    fn address_preindexed_indirect_x(&self, zp_address: &ZeroPageAddress) -> Value {
        // preindexed, add x to lookup address
        let lookup_address = zp_address.wrapping_add(self.x).upgrade();

        let address_low = self.memory.read(&lookup_address);
        let address_high = self.memory.read(&lookup_address.add(1u8));
        let address = Address::from_bytes(address_low, address_high);

        self.memory.read(&address)
    }

    fn address_postindexed_indirect_y(&self, zp_address: &ZeroPageAddress) -> (Value, u8) {
        let lookup_address = zp_address.upgrade();

        let address_low = self.memory.read(&lookup_address);
        let address_high = self.memory.read(&lookup_address.add(1u8));
        let address = Address::from_bytes(address_low, address_high);

        // post indexed, add y to lookup address
        let (address, page_crossed) = address.add_check_page_cross(self.y);

        let value = self.memory.read(&address);

        (value, page_crossed as u8)
    }
}



#[cfg(test)]
mod test {
    use crate::memory::address::{Address, AddressMode, ZeroPageAddress};
    use crate::memory::Memory;
    use crate::memory::vec_memory::VecMemory;
    use crate::processor::cmos::CmosProcessor;
    use crate::processor::{Instruction};

    #[test]
    fn test_address_immediate() {
        let mut memory = VecMemory::default();
        let mut processor = CmosProcessor::with_memory(&mut memory);

        // add small value
        processor.execute(&Instruction::ADC, &AddressMode::Immediate(5));
        assert_eq!(processor.accumulator, 5);
    }

    #[test]
    fn test_address_zeropage() {
        let zp_address = ZeroPageAddress(32);
        let address = zp_address.upgrade();
        let mut memory = VecMemory::default();
        memory.write(&address, &5);
        let mut processor = CmosProcessor::with_memory(&mut memory);

        processor.execute(&Instruction::ADC, &AddressMode::ZeroPage(zp_address));

        assert_eq!(processor.accumulator, 5);
    }

    #[test]
    fn test_address_zeropage_x() {
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
    fn test_address_absolute() {
        let address = Address(0xcabd);

        let mut memory = VecMemory::default();
        memory.write(&address, &5);

        let mut processor = CmosProcessor::with_memory(&mut memory);
        processor.execute(&Instruction::ADC, &AddressMode::Absolute(address));

        assert_eq!(processor.accumulator, 5);
    }

    #[test]
    fn test_address_absolute_x() {
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
    fn test_address_absolute_y() {
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
    fn test_address_preindexed_indirect_x() {
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
    fn test_address_postindexed_indirect_y() {
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