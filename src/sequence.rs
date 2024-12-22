use crate::u_number::UNumber;
use std::iter::Iterator;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Produces monotonously increasing integer numbers, starting from a configurable start-point.
///
/// Can be fast-forwarded to skip numbers, but cannot be wound back.
///
/// Stops producing values when the limit of the chosen type T is reached.
///
/// Can easily be written to an read from files, as it tmplements `serde::ser::Serialize` and `serde::de::Deserialize`.
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
pub struct Sequence<T> {
    o_next: Option<T>,
}

impl<T> Sequence<T>
where
    T: UNumber,
{
    /// Produces an instance that starts with 0.
    #[must_use]
    pub fn new() -> Self {
        Self {
            o_next: Some(Default::default()),
        }
    }

    /// New instance that starts with `val`.
    #[must_use]
    pub fn start_with(val: T) -> Self {
        Self { o_next: Some(val) }
    }

    /// New instance that starts with `val + 1`.
    #[must_use]
    pub fn start_after(val: T) -> Self {
        Self {
            o_next: Some(val.n_add(T::from(1_u8))),
        }
    }

    /// New instance that starts after the highest value returned by the iterator.
    pub fn start_after_highest(values: &mut dyn std::iter::Iterator<Item = T>) -> Self {
        Self::start_after(
            values
                .reduce(|x, y| std::cmp::max(x, y))
                .unwrap_or(T::from(0_u8)),
        )
    }

    /// Make sure that the Sequence will never produce the given value,
    /// by increasing the next value if necessary.
    pub fn continue_after(&mut self, val: T) {
        if val == T::max_val() {
            self.o_next = None;
        } else if let Some(ref mut next) = self.o_next {
            *next = std::cmp::max(*next, val.n_add(T::from(1_u8)));
        }
    }
}

impl<T> Iterator for Sequence<T>
where
    T: UNumber,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.o_next {
            None => None,
            Some(ref mut next) => {
                let current = *next;
                if *next == T::max_val() {
                    self.o_next = None;
                } else {
                    *next += T::from(1_u8);
                }
                Some(current)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Sequence;

    #[test]
    fn test_sequence() {
        let mut sequence = Sequence::<u8>::new();
        assert_eq!(sequence.next(), Some(0_u8));
        assert_eq!(sequence.next(), Some(1_u8));

        sequence.continue_after(5);
        assert_eq!(sequence.next(), Some(6));

        sequence.continue_after(15);
        sequence.continue_after(7);
        sequence.continue_after(0);
        assert_eq!(sequence.next(), Some(16));
    }

    #[test]
    fn test_exhaust() {
        let mut sequence = Sequence::<u64>::new();
        sequence.continue_after(u64::MAX - 1);
        assert!(sequence.next().is_some());
        assert!(sequence.next().is_none());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let mut sequence = Sequence::<u32>::start_with(55);
        assert_eq!(sequence.next(), Some(55));
        let s = serde_json::to_string(&sequence).unwrap();
        println!("{s}");
    }
}
