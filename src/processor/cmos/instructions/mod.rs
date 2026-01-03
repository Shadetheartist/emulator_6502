use std::ops::Add;
use crate::memory::address::{Address, ZeroPageAddress};
use crate::memory::Memory;
use crate::processor::cmos::CmosProcessor;
use crate::processor::Value;

pub(crate) mod adc;
mod and;

impl<'m, M: Memory> CmosProcessor<'m, M> {
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

    fn address_absolute_x(&self, address: &Address) -> (Value, bool) {
        let (address, page_crossed) = address.add_check_page_cross(self.x);
        (self.memory.read(&address), page_crossed)
    }

    fn address_absolute_y(&self, address: &Address) -> (Value, bool) {
        let (address, page_crossed) = address.add_check_page_cross(self.y);
        (self.memory.read(&address), page_crossed)
    }
    
    fn address_preindexed_indirect_x(&self, zp_address: &ZeroPageAddress) -> Value {
        // preindexed, add x to lookup address
        let lookup_address = zp_address.wrapping_add(self.x).upgrade();

        let address_low = self.memory.read(&lookup_address);
        let address_high = self.memory.read(&lookup_address.add(1u8));
        let address = Address::from_bytes(address_low, address_high);

        self.memory.read(&address)
    }
    
    fn address_postindexed_indirect_y(&self, zp_address: &ZeroPageAddress) -> (Value, bool) {
        let lookup_address = zp_address.upgrade();

        let address_low = self.memory.read(&lookup_address);
        let address_high = self.memory.read(&lookup_address.add(1u8));
        let address = Address::from_bytes(address_low, address_high);

        // post indexed, add y to lookup address
        let (address, page_crossed) = address.add_check_page_cross(self.y);

        let value = self.memory.read(&address);

        (value, page_crossed)
    }
}