use std::{
    collections::HashSet,
    hash::Hash,
    iter::Product,
    ops::{Div, Mul, Rem},
};

use num_traits::{One, Zero};

use crate::factors::{FactorType, Factors};

pub trait FractionType:
    Copy
    + Sized
    + Div<Output = Self>
    + Mul<Self, Output = Self>
    + Rem<Output = Self>
    + PartialEq
    + Zero
    + FactorType
    + Eq
    + Hash
    + Ord
    + One
{
    /// Returns the value of 10 typed in FractionType
    fn ten() -> Self;
}

#[derive(Clone, Debug)]
pub struct Fraction<T: FractionType> {
    pub numerator: T,
    pub denominator: T,
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

    pub fn reduce(&self) -> Self {
        let numerator_factors = Factors::new(&self.numerator).collect::<HashSet<_>>();
        let denominator_factors = Factors::new(&self.denominator).collect::<HashSet<_>>();

        let mut common_factors = numerator_factors
            .intersection(&denominator_factors)
            .collect::<Vec<_>>();
        common_factors.sort();
        common_factors.reverse();

        if common_factors.len() == 0 {
            return self.clone();
        }

        let greatest_common_factor = *common_factors[0];
        if greatest_common_factor == One::one() {
            return self.clone();
        }
        return Fraction::new(
            self.numerator / greatest_common_factor,
            self.denominator / greatest_common_factor,
        );
    }
}

impl<T: FractionType> PartialEq for Fraction<T> {
    fn eq(&self, other: &Self) -> bool {
        let reduced_self = self.reduce();
        let reduced_other = other.reduce();

        reduced_self.numerator == reduced_other.numerator
            && reduced_self.denominator == reduced_other.denominator
    }
}

impl<T: FractionType> PartialOrd for Fraction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.denominator == other.denominator {
            return self.numerator.partial_cmp(&other.numerator);
        }
        let multiple = self.denominator * other.denominator;
        let s = match self.denominator {
            _ if self.denominator == multiple => self.clone(),
            _ => Fraction::new(self.numerator * multiple, multiple),
        };
        let o = match other.denominator {
            _ if other.denominator == multiple => other.clone(),
            _ => Fraction::new(other.numerator * multiple, multiple),
        };
        s.partial_cmp(&o)
    }
}

impl<T: FractionType> From<T> for Fraction<T> {
    fn from(value: T) -> Self {
        Self::new(value, value)
    }
}

impl<'a, T: FractionType + 'a> Product<&'a Fraction<T>> for Fraction<T> {
    fn product<I: Iterator<Item = &'a Fraction<T>>>(iter: I) -> Self {
        iter.cloned()
            .reduce(|acc, n| acc * n)
            .unwrap_or_else(|| Fraction::new(Zero::zero(), One::one()))
    }
}

impl<T: FractionType> Mul<Fraction<T>> for Fraction<T> {
    type Output = Fraction<T>;

    fn mul(self, other: Fraction<T>) -> Self::Output {
        Fraction::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
        .reduce()
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

    #[test]
    fn test_reduce_already_reduced() {
        let fraction = Fraction::new(1u8, 2u8);
        assert_eq!(fraction, fraction.reduce());
    }

    #[test]
    fn test_reduce() {
        let fraction = Fraction::new(2u8, 4u8);
        assert_eq!(Fraction::new(1u8, 2u8), fraction.reduce())
    }
}
