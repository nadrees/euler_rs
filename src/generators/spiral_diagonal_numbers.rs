use std::ops::AddAssign;

use num_traits::Num;

use crate::{Three, Two};

pub trait SpiralDiagonalNumberType: Num + Two + Three + Copy + PartialOrd + AddAssign {}

impl<T> SpiralDiagonalNumberType for T where T: Num + Two + Three + Copy + PartialOrd + AddAssign {}

/// A spiral diagonal number is number that is on the
/// corner of an N x N grid created by starting at 1,
/// and then moving clockwise. For example, a 5x5
/// spiral grid is:
///
/// 21 22 23 24 25
/// 20  7  8  9 10
/// 19  6  1  2 11
/// 18  5  4  3 12
/// 17 16 15 14 13
///
/// The spiral diagonal numbers for this grid are:
/// 1, 3, 5, 7, 9, 13, 17, 21, 25
///
/// This generator can produce the spiral diagonal
/// numbers for the N x N grid.
///
/// NOTE: N must be odd
pub struct SpiralDiagonalNumbers<T: SpiralDiagonalNumberType> {
    max_value: T,
    iter: SpiralDiagonalNumberRotationGenerator<T>,
}

impl<T: SpiralDiagonalNumberType> SpiralDiagonalNumbers<T> {
    pub fn new(n: T) -> Self {
        assert!(n % T::two() == T::one(), "n must be odd");
        SpiralDiagonalNumbers {
            max_value: n * n,
            iter: SpiralDiagonalNumberRotationGenerator::new(T::zero()),
        }
    }
}

impl<T: SpiralDiagonalNumberType> Iterator for SpiralDiagonalNumbers<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => {
                self.iter =
                    SpiralDiagonalNumberRotationGenerator::new(self.iter.rotation + T::one());
                return self.next();
            }
            Some(next) if next <= self.max_value => Some(next),
            _ => None,
        }
    }
}

/// Generates the sprial diagonal numbers
/// for a given 'rotation' around the spiral.
/// Rotation = 0 is the center most position (1),
/// and Rotation = 1 is the clockwise circle starting at 2 and ending with 9
struct SpiralDiagonalNumberRotationGenerator<T: SpiralDiagonalNumberType> {
    rotation: T,
    counter: T,
    amount_to_start_from: T,
    amount_to_advance_by: T,
}

impl<T: SpiralDiagonalNumberType> SpiralDiagonalNumberRotationGenerator<T> {
    pub fn new(rotation: T) -> Self {
        let amount_to_advance_by = if rotation == T::zero() {
            T::one()
        } else {
            rotation * T::two()
        };

        let amount_to_start_from = if rotation == T::zero() {
            T::zero()
        } else {
            let n = (T::two() * rotation) - T::one();
            n * n
        };

        SpiralDiagonalNumberRotationGenerator {
            rotation,
            counter: T::zero(),
            amount_to_start_from,
            amount_to_advance_by,
        }
    }
}

impl<T: SpiralDiagonalNumberType> Iterator for SpiralDiagonalNumberRotationGenerator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.rotation == T::zero() && self.counter > T::zero()) || self.counter > T::three() {
            return None;
        }

        self.counter += T::one();
        return Some(self.counter * self.amount_to_advance_by + self.amount_to_start_from);
    }
}

#[cfg(test)]
mod spiral_diagonal_number_rotation_generator_tests {
    use super::*;

    #[test]
    fn test_new() {
        let generator = SpiralDiagonalNumberRotationGenerator::new(0);
        assert_eq!(generator.rotation, 0);
        assert_eq!(generator.amount_to_start_from, 0);
        assert_eq!(generator.amount_to_advance_by, 1);

        let generator = SpiralDiagonalNumberRotationGenerator::new(1);
        assert_eq!(generator.rotation, 1);
        assert_eq!(generator.amount_to_start_from, 1);
        assert_eq!(generator.amount_to_advance_by, 2);

        let generator = SpiralDiagonalNumberRotationGenerator::new(2);
        assert_eq!(generator.rotation, 2);
        assert_eq!(generator.amount_to_start_from, 9);
        assert_eq!(generator.amount_to_advance_by, 4);
    }

    #[test]
    fn test_generator() {
        let generator = SpiralDiagonalNumberRotationGenerator::new(0);
        assert_eq!(vec![1], generator.collect::<Vec<_>>());

        let generator = SpiralDiagonalNumberRotationGenerator::new(1);
        assert_eq!(vec![3, 5, 7, 9], generator.collect::<Vec<_>>());

        let generator = SpiralDiagonalNumberRotationGenerator::new(2);
        assert_eq!(vec![13, 17, 21, 25], generator.collect::<Vec<_>>());
    }
}

#[cfg(test)]
mod sprial_diagonal_number_generator_tests {
    use super::*;

    #[test]
    fn test_generator() {
        let generator = SpiralDiagonalNumbers::new(5);
        assert_eq!(
            vec![1, 3, 5, 7, 9, 13, 17, 21, 25],
            generator.collect::<Vec<_>>()
        );
    }
}
