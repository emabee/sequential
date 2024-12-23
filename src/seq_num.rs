use std::ops::{AddAssign, SubAssign};

pub trait SeqNum:
    Default + Copy + Ord + AddAssign + SubAssign + From<u8> + TryFrom<u128> + Into<u128> + PartialEq
{
    fn max_val() -> Self;
    fn n_add(&self, other: Self) -> Self;
}
impl SeqNum for u8 {
    fn max_val() -> Self {
        u8::MAX
    }
    fn n_add(&self, other: Self) -> Self {
        self + other
    }
}
impl SeqNum for u16 {
    fn max_val() -> Self {
        u16::MAX
    }
    fn n_add(&self, other: Self) -> Self {
        self + other
    }
}
impl SeqNum for u32 {
    fn max_val() -> Self {
        u32::MAX
    }
    fn n_add(&self, other: Self) -> Self {
        self + other
    }
}
impl SeqNum for u64 {
    fn max_val() -> Self {
        u64::MAX
    }
    fn n_add(&self, other: Self) -> Self {
        self + other
    }
}
impl SeqNum for u128 {
    fn max_val() -> Self {
        u128::MAX
    }
    fn n_add(&self, other: Self) -> Self {
        self + other
    }
}
