use enum_as_inner::EnumAsInner;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, TryFromPrimitive, EnumAsInner,
)]
#[repr(u8)]
pub enum Command {
    ACT = 0,
    PRE,
    PREA,
    RD,
    WR,
    RDA,
    WRA,
    REF,
    PDE,
    PDX,
    SRE,
    SRX,
    Max,
}
