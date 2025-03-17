use bitflags::bitflags;
bitflags! {
    pub struct Corners: u32 {
        const NONE = 0;
        const TOP_RIGHT = 1;
        const TOP_LEFT = 2;
        const BOTTOM_LEFT = 4;
        const BOTTOM_RIGHT = 8;
        const ALL = Self::TOP_RIGHT.bits() |  Self::BOTTOM_RIGHT.bits() | Self::BOTTOM_LEFT.bits() | Self::TOP_LEFT.bits();
    }
}
bitflags! {
    pub struct CenterSymbols: u32 {
        const NONE = 0;
        const TOP_CENTER = 1;
        const BOTTOM_CENTER = 2;
        const LEFT_CENTER = 4;
        const RIGHT_CENTER = 8;
        const ALL = Self::TOP_CENTER.bits() | Self::RIGHT_CENTER.bits() | Self::BOTTOM_CENTER.bits() | Self::LEFT_CENTER.bits();
    }
}
bitflags! {
    struct RepBorderSymbols: u32 {
        const TOP_VERTICAL_RIGHT    = 1;
        const TOP_VERTICAL_LEFT     = 2;
        const BOTTOM_VERTICAL_RIGHT = 4;
        const BOTTOM_VERTICAL_LEFT  = 8;
        const TOP_HORIZONTAL_RIGHT  = 16;
        const TOP_HORIZONTAL_LEFT   = 32;
        const BOTTOM_HORIZONTAL_RIGHT = 64;
        const BOTTOM_HORIZONTAL_LEFT  = 128;
    }
}
