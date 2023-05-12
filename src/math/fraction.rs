use std::ops::{Div, Mul, Rem};

use num_traits::Zero;

pub trait FractionType:
    Copy + Sized + Div<Output = Self> + Mul<Self, Output = Self> + Rem<Output = Self> + PartialEq + Zero
{
    /// Returns the value of 10 typed in FractionType
    fn ten() -> Self;
}

pub struct Fraction<T: FractionType> {
    numerator: T,
    denominator: T,
}

impl<T: FractionType> Fraction<T> {
    pub fn new(numerator: T, denominator: T) -> Fraction<T> {
        Fraction {
            numerator,
            denominator,
        }
    }

    /// Returns the whole portion of this fraction
    pub fn whole_portion(self) -> T {
        self.numerator / self.denominator
    }

    /// Returns an iterator over the (potentially) infinte digits after the decimal point
    pub fn decimal_portion(self) -> impl Iterator<Item = T> {
        let remainder = self.numerator % self.denominator;
        DecimalFormIterator {
            numerator: remainder,
            denominator: self.denominator,
        }
    }
}

struct DecimalFormIterator<T: FractionType> {
    numerator: T,
    denominator: T,
}

impl<T: FractionType> Iterator for DecimalFormIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.numerator == Zero::zero() {
            return None;
        }
        let numerator = self.numerator * FractionType::ten();
        let answer = numerator / self.denominator;
        let remainder = numerator % self.denominator;
        self.numerator = remainder;
        return Some(answer);
    }
}

macro_rules! fraction_type_impl {
    ($T:ty, $ten:expr) => {
        impl FractionType for $T {
            #[inline]
            fn ten() -> $T {
                $ten
            }
        }
    };
}

fraction_type_impl!(u8, 10);
fraction_type_impl!(u16, 10);
fraction_type_impl!(u32, 10);
fraction_type_impl!(u64, 10);
fraction_type_impl!(u128, 10);
fraction_type_impl!(usize, 10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whole_portion() {
        let fraction = Fraction::new(1u8, 2);
        assert_eq!(0, fraction.whole_portion());

        let fraction = Fraction::new(4u16, 2);
        assert_eq!(2, fraction.whole_portion());

        let fraction = Fraction::new(3u32, 2);
        assert_eq!(1, fraction.whole_portion());
    }

    #[test]
    fn test_decimal_portion() {
        let fraction = Fraction::new(1u64, 2);
        assert_eq!(vec![5], fraction.decimal_portion().collect::<Vec<_>>())
    }
}
