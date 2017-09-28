// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/computor-v1
//
// This file may not be copied, modified, or distributed

//! This module `Degree`'s definition.

use ::fmt;
use ::str;
use ::num;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Degree(pub i32);

impl Default for Degree {
    fn default() -> Self {
        Degree(1)
    }
}

impl str::FromStr for Degree {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(power) = s.find('^') {
            let (_, degree): (&str, &str) = s.split_at(power+1);

            Ok(Degree(try!(i32::from_str(degree))))
        } else {
            Ok(Degree::default())
        }
    }
}

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Degree(1) => Ok(()),
            Degree(power) => write!(f, "^{}", power),
        }
    }
}
