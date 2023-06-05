use bitflags::bitflags;

#[rustfmt::skip]
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Side: u8 {
        const FRONT     = 0b00000001;
        const BACK      = 0b00000010;
        const LEFT      = 0b00000100;
        const RIGHT     = 0b00001000;
        const TOP       = 0b00010000;
        const BOTTOM    = 0b00100000;
    }
}
