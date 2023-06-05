use bitflags::bitflags;

#[rustfmt::skip]
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CubeSide: u8 {
        const UP    = 0b00000001;
        const DOWN  = 0b00000010;
        const LEFT  = 0b00000100;
        const RIGHT = 0b00001000;
        const FRONT = 0b00010000;
        const BACK  = 0b00100000;
    }
}
