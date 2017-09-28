// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/computor-v1
//
// This file may not be copied, modified, or distributed

//! This module `order`'s interface is a list of term.

pub mod term;

use self::term::Term;
use self::term::degree::Degree;

use ::fmt;
use ::str;
use ::vec;
use ::ops::{Not, Neg, Add, Rem};
use ::num;
use ::array;
use ::iter;

use itertools::Itertools;
use num2::Zero;

/// The structure Order is a list of Terms.
#[derive(Debug, Clone, PartialEq)]
pub struct Order(pub Vec<Term>);

impl Order {
    fn new(it: Vec<Term>) -> Self {
        Order(it.into_iter()
                .sorted()
                .into_iter()
                .group_by(|elt: &Term| match elt {
                      &Term::Determinate(_) => false,
                      &Term::Indeterminate(_, Degree(power)) => power.rem(2).eq(&1),
                })
                .into_iter()
                .map(|(_, group): (bool, ::itertools::Group<bool, vec::IntoIter<Term>, _>)| group.fold(Term::default(), |acc, term| term.add(acc)))
                .filter(|term| term.is_zero().not())
                .collect::<Vec<Term>>())
    }
}

impl str::FromStr for Order {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.replace(" ", "").replace("*", "").replace("-", "+-");
        if let Some(position) = line.find('=') {
            let (left, rright): (&str, &str) = line.split_at(position);
            let (_, right): (&str, &str) = rright.split_at(1);
    
            Ok(Order::new(left.split("+")
                              .filter_map(|term| Term::from_str(term).ok())
                              .chain(right.split("+")
                                          .filter_map(|term| Term::from_str(term).ok())
                                          .map(|term| term.neg()))
                              .collect::<Vec<Term>>()))
        } else {
            Ok(Order::new(line.split("+")
                              .filter_map(|term| Term::from_str(term).ok())
                              .collect::<Vec<Term>>()))
        }
    }
}

impl Default for Order {
    fn default() -> Self {
        Order(Vec::new())
    }
}

impl Iterator for Order {
    type Item = Term;

    fn next(&mut self) -> Option<Term> {
        None
    }
}

impl iter::ExactSizeIterator for Order {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

unsafe impl array::FixedSizeArray<Term> for Order {
    fn as_slice(&self) -> &[Term] {
        self.0.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [Term] {
        self.0.as_mut_slice()
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
             write!(f, "0 = 0")
        } else if self.0.len().eq(&1) {
             write!(f, "{} != 0", self.0.first().unwrap_or(&Term::default()))
        } else {
             write!(f, "{} = 0", self.0.iter()
                                      .map(|term| format!("{}", term))
                                      .collect::<Vec<String>>()
                                      .join(" + ")
                                      .replace("+ -", "- "))
        }
    }
}
