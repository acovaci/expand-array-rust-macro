pub struct BitifyFlags(u8);

impl BitifyFlags {
    pub const ESCAPING: u8 = 0;
    pub const START: u8 = 1;
    pub const END: u8 = 2;

    pub const fn new() -> Self {
        Self(0b0000_0000)
    }

    pub const fn mask(&self, index: u8) -> u8 {
        1 << index
    }

    pub const fn set_on(self, index: u8) -> Self {
        let flags = self.0 | self.mask(index);
        Self(flags)
    }

    pub const fn toggle(self, index: u8) -> Self {
        let flags = self.0 ^ self.mask(index);
        Self(flags)
    }

    pub const fn is_on(&self, index: u8) -> bool {
        self.0 & self.mask(index) != 0
    }

    pub const fn is_off(&self, index: u8) -> bool {
        !self.is_on(index)
    }
}
