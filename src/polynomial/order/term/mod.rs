// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/computor-v1
//
// This file may not be copied, modified, or distributed

//! This module `Term`'s definition.

pub mod degree;

use self::degree::Degree;

use ::fmt;
use ::str;
use ::cmp;
use ::ops::{self, Sub, Neg, BitAnd};
use ::num;
use ::num2;

#[derive(Debug, Clone, Copy)]
pub enum Term {
    Determinate(f64),
    Indeterminate(f64, Degree),
}

impl Default for Term {
    fn default() -> Self {
        Term::Determinate(f64::default())
    }
}

impl Neg for Term {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Term::Determinate(number) => Term::Determinate(number.neg()),
            Term::Indeterminate(number, power) => Term::Indeterminate(number.neg(), power),
        }
    }
}

impl str::FromStr for Term {
    type Err = num::ParseFloatError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some(constant) = line.find('X') {
            let (left, right): (&str, &str) = line.split_at(constant);
            match Degree::from_str(right) {
                Ok(Degree(0)) => Ok(Term::Determinate(try!(f64::from_str(left)))),
                power => Ok(Term::Indeterminate(try!(f64::from_str(left)), power.unwrap_or_default())),
            }
        } else {
            if let Ok(Degree(power)) = Degree::from_str(line) {
                Ok(Term::Determinate(try!(f64::from_str(line)).powi(power)))
            } else {
                Ok(Term::Determinate(try!(f64::from_str(line))))
            }
        }
    }
}

impl Eq for Term {}

impl cmp::PartialEq for Term {
    fn eq(&self, other: &Term) -> bool {
        match (self, other) {
            (&Term::Determinate(left), &Term::Determinate(right)) if left.eq(&right) => true,
            (&Term::Indeterminate(left, left_power), &Term::Indeterminate(right, right_power))
                if left.eq(&right).bitand(left_power.eq(&right_power)) => true,
            _ => false,
        }
    }
}

impl cmp::Ord for Term {
    fn cmp(&self, other: &Term) -> cmp::Ordering {
        //! https://github.com/rust-lang/rfcs/issues/1249
        match (self, other) {
            (&Term::Determinate(right), &Term::Determinate(left)) => left.partial_cmp(&right).unwrap_or(cmp::Ordering::Equal),
            (&Term::Determinate(..), &Term::Indeterminate(..)) => cmp::Ordering::Greater,
            (&Term::Indeterminate(..), &Term::Determinate(..)) => cmp::Ordering::Less,
            (&Term::Indeterminate(right, Degree(right_power)), &Term::Indeterminate(left, Degree(left_power)))
                 => left_power.cmp(&right_power).then(left.partial_cmp(&right).unwrap_or(cmp::Ordering::Equal)),
        }
    }
}

impl cmp::PartialOrd for Term {
    fn partial_cmp(&self, other: &Term) -> Option<cmp::Ordering> {
        //! https://github.com/rust-lang/rfcs/issues/1249
        match (self, other) {
            (&Term::Determinate(right), &Term::Determinate(left)) => left.partial_cmp(&right),
            (&Term::Determinate(..), &Term::Indeterminate(..)) => Some(cmp::Ordering::Greater),
            (&Term::Indeterminate(..), &Term::Determinate(..)) => Some(cmp::Ordering::Less),
            (&Term::Indeterminate(right, Degree(right_power)), &Term::Indeterminate(left, Degree(left_power)))
                => Some(left_power.cmp(&right_power).then(left.partial_cmp(&right).unwrap_or(cmp::Ordering::Equal)))
        }
    }
}

impl ops::Sub for Term {
    type Output = Self;

    fn sub(self, other: Term) -> Self::Output {
        match (self, other) {
            (Term::Determinate(left), Term::Determinate(right)) => {
                Term::Determinate(left.sub(right))
            },
            (Term::Indeterminate(left, left_power), Term::Indeterminate(right, _)) => {
                Term::Indeterminate(left.sub(right), left_power)
            },
            (first, _) => first,
        }
    }
}

impl num2::CheckedSub for Term {
    fn checked_sub(&self, other: &Term) -> Option<Self::Output> {
        match (self, other) {
            (&Term::Determinate(left), &Term::Determinate(right)) => {
                Some(Term::Determinate(left.sub(right)))
            },
            (&Term::Indeterminate(right, right_power), &Term::Indeterminate(left, left_power))
                if left_power == right_power => {
                    Some(Term::Indeterminate(left.sub(right), left_power))
            },
            (first, _) => Some(*first),
        }
    }
}

impl ops::Add for Term {
    type Output = Self;

    fn add(self, other: Term) -> Self::Output {
        match (self, other) {
            (Term::Determinate(left), Term::Determinate(right)) => {
                Term::Determinate(left.add(right))
            },
            (Term::Indeterminate(left, left_power), Term::Indeterminate(right, _)) => {
                Term::Indeterminate(left.add(right), left_power)
            },
            (first, _) => first,
        }
    }
}

impl num2::Zero for Term {
    fn zero() -> Self {
        Term::default()
    }

    fn is_zero(&self) -> bool {
        match self {
            &Term::Determinate(number) if number.eq(&f64::zero()) => true,
            &Term::Indeterminate(number, _) if number.eq(&f64::zero()) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Term::Determinate(number) => write!(f, "{}", number),
            &Term::Indeterminate(number, power) => write!(f, "{}x{}", number, power),
        }
    }
}
