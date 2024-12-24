/// Helper trait to deal generically with built-in types.
pub trait SeqNum: Copy + Ord + PartialEq {
    /// Returns the max value of the type.
    fn max_val() -> Self;
    /// Calls `checked_add` of the type.
    fn checked_add(self, other: Self) -> Option<Self>;
    /// Returns 0.
    fn zero() -> Self;
    /// Returns 1.
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
