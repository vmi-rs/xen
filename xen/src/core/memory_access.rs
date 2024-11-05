bitflags::bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct MemoryAccess: u8 {
        const NONE      = 0b00000000;
        const R         = 0b00000001;
        const W         = 0b00000010;
        const X         = 0b00000100;
        const RW        = Self::R.bits() | Self::W.bits();
        const WX        = Self::W.bits() | Self::X.bits();
        const RX        = Self::R.bits() | Self::X.bits();
        const RWX       = Self::R.bits() | Self::W.bits() | Self::X.bits();

        const RX2RW     = 0b00001000;
        const N2RWX     = 0b00001001;
        const DEFAULT   = 0b00001010;
    }
}
