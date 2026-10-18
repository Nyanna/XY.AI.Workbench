//! Pointer-free memory handle; resolution to a device address is backend-internal.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArenaId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Region {
    pub arena: ArenaId,
    pub offset: u64,
    pub len: u64,
    pub gen: u32,
}
