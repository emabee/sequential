use crate::seq_num::SeqNum;
use std::iter::Iterator;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A number generator that produces monotonously increasing integer numbers,
/// starting from a configurable start-point.
///
/// Can be fast-forwarded to skip numbers, but cannot be wound back.
///
/// Passivates itself when the limit of the chosen type `T` is reached.
/// Passive instances do not produce values anymore.
///
/// Optionally (with feature `serde`) implements `serde::Serialize` and `serde::Deserialize`.
///
/// Works with all unsigned integers, from `u8` to `u128`.
///
/// ## Example:
///
/// ```rust
/// use sequential::Sequence;
/// {
///     let mut sequence = Sequence::<u8>::new();
///     assert_eq!(sequence.next(), Some(0_u8));
///     assert_eq!(sequence.next(), Some(1_u8));
///
///     sequence.continue_after(5);
///     assert_eq!(sequence.next(), Some(6));
///
///     sequence.continue_after(15);
///     sequence.continue_after(7);
///     sequence.continue_after(0);
///     assert_eq!(sequence.next(), Some(16));
/// }
///
/// {
///     let mut sequence = Sequence::<u64>::new();
///     sequence.continue_after(u64::MAX-1);
///     assert!(sequence.next().is_some());
///     assert!(sequence.next().is_none());
/// }
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct Sequence<T>
where
    T: SeqNum,
{
    next: T,
    // if > 0: the increment; if == 0: the instance is passivated
    incr: T,
    #[cfg_attr(feature = "serde", serde(default = "SeqNum::max_val"))]
    max: T,
}

impl<T> Sequence<T>
where
    T: SeqNum,
{
    /// Produces an instance that starts with 0 and increments by 1.
    #[must_use]
    pub fn new() -> Self {
        Self {
            next: T::zero(),
            incr: T::one(),
            max: T::max_val(),
        }
    }

    // Produces a dead instance, good for nothing.
    #[must_use]
    fn dead() -> Self {
        Self {
            next: T::zero(),
            incr: T::zero(),
            max: T::max_val(),
        }
    }

    /// Produces an instance that starts with `val` and increments by 1.
    #[must_use]
    pub fn start_with(val: T) -> Self {
        Self {
            next: val,
            incr: T::one(),
            max: T::max_val(),
        }
    }

    /// Produces an instance that starts with `val + 1` and increments by 1.
    #[must_use]
    pub fn start_after(val: T) -> Self {
        match val.checked_add(T::one()) {
            Some(next) => Self {
                next,
                incr: T::one(),
                max: T::max_val(),
            },
            None => Self::dead(),
        }
    }

    /// Produces an instance that starts after the highest value returned by the iterator.
    pub fn start_after_highest(values: &mut dyn Iterator<Item = T>) -> Self {
        Self::start_after(
            values
                .reduce(|x, y| std::cmp::max(x, y))
                .unwrap_or(T::zero()),
        )
    }

    /// Produces an instance with explicitly configured upper limit.
    pub fn with_start_end_increment(start: T, end: T, incr: T) -> Self {
        Self {
            next: start,
            incr,
            max: end,
        }
    }

    /// Consumes the Sequence and produces one that increments with the given value.
    ///
    /// An increment of `0` produces a dead sequence that will not return any value.
    ///
    /// Note: the new increment takes effect _after_ the next value, not with the next value.
    /// This is irrelevant if you call this method before consuming any value.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use sequential::Sequence;
    /// let mut sequence = Sequence::<usize>::new().with_increment(5);
    /// assert_eq!(sequence.next(), Some(0));
    /// assert_eq!(sequence.next(), Some(5));
    /// assert_eq!(sequence.next(), Some(10));
    /// ```
    #[must_use]
    pub fn with_increment(mut self, incr: T) -> Self {
        if self.is_active() {
            self.incr = incr;
        }
        self
    }

    /// Make sure that the Sequence will never produce the given value,
    /// by increasing the next value if necessary.
    pub fn continue_after(&mut self, val: T) {
        match val.checked_add(self.incr) {
            Some(candidate) => {
                self.next = std::cmp::max(self.next, candidate);
            }
            None => {
                self.set_passive();
            }
        }
    }

    fn set_passive(&mut self) {
        self.incr = T::zero();
    }
    fn is_active(&self) -> bool {
        self.incr != T::zero()
    }
    fn is_passive(&self) -> bool {
        self.incr == T::zero()
    }
}

/// Allows using Sequence for iterations.
///
/// Example
///
/// ```rust
/// # use sequential::Sequence;
/// assert_eq!(Sequence::<u8>::with_start_end_increment(23, 38, 3).sum::<u8>(),183);
/// ```
impl<T> Iterator for Sequence<T>
where
    T: SeqNum,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next > self.max {
            self.set_passive();
        }

        if self.is_passive() {
            None
        } else {
            let current = self.next;
            match self.next.checked_add(self.incr) {
                Some(next) => {
                    self.next = next;
                }
                None => {
                    self.set_passive();
                }
            }
            Some(current)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Sequence;

    #[test]
    fn test_sequence() {
        let mut sequence = Sequence::<usize>::new();
        assert_eq!(sequence.next(), Some(0_usize));
        assert_eq!(sequence.next(), Some(1_usize));

        sequence.continue_after(5);
        assert_eq!(sequence.next(), Some(6));

        sequence.continue_after(15);
        sequence.continue_after(7);
        sequence.continue_after(0);
        assert_eq!(sequence.next(), Some(16));
    }

    #[test]
    fn test_increment() {
        let mut sequence = Sequence::<u8>::new().with_increment(5);
        assert_eq!(sequence.next(), Some(0));
        assert_eq!(sequence.next(), Some(5));
        assert_eq!(sequence.next(), Some(10));

        sequence.continue_after(152);
        assert_eq!(sequence.next(), Some(157));
        assert_eq!(sequence.next(), Some(162));

        sequence.continue_after(251);
        assert_eq!(sequence.next(), None);
    }

    #[test]
    fn test_exhaust() {
        let mut sequence = Sequence::<u64>::new();
        sequence.continue_after(u64::MAX - 1);
        assert!(sequence.is_active());
        assert!(sequence.next().is_some());
        assert!(sequence.next().is_none());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let mut sequence = Sequence::<u32>::with_start_end_increment(22, 99, 11);
        assert_eq!(sequence.next(), Some(22));
        let s = serde_json::to_string(&sequence).unwrap();
        assert_eq!(&*s, r#"{"next":33,"incr":11,"max":99}"#);

        let mut sequence2: Sequence<u32> = serde_json::from_str(&*s).unwrap();
        assert_eq!(sequence2.next(), Some(33));
        assert_eq!(sequence2.next(), Some(44));
        assert_eq!(sequence2.next(), Some(55));
        assert_eq!(sequence2.next(), Some(66));
        assert_eq!(sequence2.next(), Some(77));
        assert_eq!(sequence2.next(), Some(88));
        assert_eq!(sequence2.next(), Some(99));
        assert_eq!(sequence2.next(), None);

        // compatibility to old serialization format (without max)
        let old_format = r#"{"next":88,"incr":11}"#;
        let mut sequence3: Sequence<u32> = serde_json::from_str(&old_format).unwrap();
        assert_eq!(sequence3.next(), Some(88));
        assert_eq!(sequence3.next(), Some(99));
        assert_eq!(sequence3.next(), Some(110));
        assert_eq!(sequence3.next(), Some(121));
    }

    #[test]
    fn test_iter() {
        assert_eq!(
            Sequence::<u8>::with_start_end_increment(23, 38, 3).sum::<u8>(),
            183
        );
    }
}
