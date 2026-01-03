use std::ops::Add;
use crate::processor::Value;

#[derive(Debug, Clone, Copy)]
pub struct ZeroPageAddress(pub u8);

impl ZeroPageAddress {
    pub fn wrapping_add(self, rhs: u8) -> Self {
        ZeroPageAddress(self.0.wrapping_add(rhs))
    }
}

impl ZeroPageAddress {
    pub fn upgrade(&self) -> Address {
        Address(self.0 as u16)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Address(pub u16);

impl Address {
    pub fn from_bytes(low: u8, high: u8) -> Self {
        Address(((high as u16) << 8) | low as u16)
    }

    pub fn add_check_page_cross(self, rhs: u8) -> (Self, bool) {
        // isolate upper byte into tmp var
        let upper: u8 = (self.0 >> 8) as u8;
        // compute sum
        let sum = self.0.wrapping_add(rhs as u16);
        // compare sum upper byte to see if the page boundary was crossed
        let changed = (sum >> 8) as u8 != upper;

        (Self(sum), changed)
    }
}

impl Add<u8> for Address {
    type Output = Address;

    fn add(self, rhs: u8) -> Self::Output {
        Address(self.0.wrapping_add(rhs as u16))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AddressMode {
    Immediate(Value),
    ZeroPage(ZeroPageAddress),
    ZeroPageX(ZeroPageAddress),
    ZeroPageY(ZeroPageAddress),
    Absolute(Address),
    AbsoluteX(Address),
    AbsoluteY(Address),
    Indirect(Address),                    // ($ff22)
    PreIndexedIndirectX(ZeroPageAddress),  // (Zero-Page,X)
    PostIndexedIndirectY(ZeroPageAddress), // (Zero-Page), Y
    Relative(Address),
}


#[cfg(test)]
mod test {
    use std::ops::Add;
    use crate::memory::address::Address;

    #[test]
    fn test_from_bytes(){
        let address = Address::from_bytes(0xca, 0xbb);
        assert_eq!(address.0, 0xbbca)
    }

    #[test]
    fn test_add(){
        let address = Address(0x1000).add(0x0001);
        assert_eq!(address.0, 0x1001)
    }

    #[test]
    fn test_add_page_check(){
        let (address, crossed) = Address(0x00ff).add_check_page_cross(0x0001);
        assert_eq!(address.0, 0x0100);
        assert_eq!(crossed, true);

        let (address, crossed) = Address(0x01fa).add_check_page_cross(0x0001);
        assert_eq!(address.0, 0x01fb);
        assert_eq!(crossed, false);
    }
}