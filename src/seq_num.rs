pub trait SeqNum: Copy + Ord + PartialEq {
    fn max_val() -> Self;
    fn checked_add(self, other: Self) -> Option<Self>;
    fn zero() -> Self;
    fn one() -> Self;
}
impl SeqNum for usize {
    fn max_val() -> Self {
        usize::MAX
    }
    fn checked_add(self, other: Self) -> Option<Self> {
        usize::checked_add(self, other)
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl SeqNum for u8 {
    fn max_val() -> Self {
        u8::MAX
    }
    fn checked_add(self, other: Self) -> Option<Self> {
        u8::checked_add(self, other)
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl SeqNum for u16 {
    fn max_val() -> Self {
        u16::MAX
    }
    fn checked_add(self, other: Self) -> Option<Self> {
        u16::checked_add(self, other)
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl SeqNum for u32 {
    fn max_val() -> Self {
        u32::MAX
    }
    fn checked_add(self, other: Self) -> Option<Self> {
        u32::checked_add(self, other)
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl SeqNum for u64 {
    fn max_val() -> Self {
        u64::MAX
    }
    fn checked_add(self, other: Self) -> Option<Self> {
        u64::checked_add(self, other)
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
impl SeqNum for u128 {
    fn max_val() -> Self {
        u128::MAX
    }
    fn checked_add(self, other: Self) -> Option<Self> {
        u128::checked_add(self, other)
    }
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}
