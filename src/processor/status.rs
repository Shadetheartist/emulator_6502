use crate::processor::Register8;

pub const FLAG_NEGATIVE: u8 = 7;
pub const FLAG_OVERFLOW: u8 = 6;
pub const FLAG_UNUSED_5: u8 = 5;
pub const FLAG_BREAK: u8 = 4;
pub const FLAG_DECIMAL: u8 = 3;
pub const FLAG_INTERRUPT_DISABLE: u8 = 2;
pub const FLAG_ZERO: u8 = 1;
pub const FLAG_CARRY: u8 = 0;


#[derive(Default)]
pub struct Status(pub(crate) Register8);

impl Status {
    #[inline(always)]
    pub fn get_bit(&self, n: u8) -> bool {
        self.get_bit_u8(n) == 1
    }

    #[inline(always)]
    pub fn get_bit_u8(&self, n: u8) -> u8 {
        self.0 >> n & 1
    }

    #[inline(always)]
    pub fn clear_bit(&mut self, n: u8) {
        self.0 = self.0 & !(1u8 << n);
    }

    #[inline(always)]
    pub fn enable_bit(&mut self, n: u8) {
        self.set_bit(n, true)
    }

    pub fn set_bit(&mut self, n: u8, value: bool) {
        self.0 = self.0 | ((value as u8) << n);
    }
}

#[cfg(test)]
mod test {
    use crate::processor::status::{Status, FLAG_NEGATIVE, FLAG_OVERFLOW};

    #[test]
    fn test_status() {
        let mut status = Status::default();
        assert_eq!(status.get_bit(FLAG_NEGATIVE), false);

        status.enable_bit(FLAG_NEGATIVE);
        assert_eq!(status.get_bit(FLAG_NEGATIVE), true);
        assert_eq!(status.0, 128);

        status.enable_bit(FLAG_OVERFLOW);
        status.clear_bit(FLAG_NEGATIVE);
        assert_eq!(status.get_bit(FLAG_NEGATIVE), false);
        assert_eq!(status.get_bit(FLAG_OVERFLOW), true);
        assert_eq!(status.0, 64);
    }
}
