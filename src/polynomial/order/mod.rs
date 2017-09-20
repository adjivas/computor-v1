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

#[derive(Debug, Clone, PartialEq)]
pub struct Order(pub Vec<Term>);

impl str::FromStr for Order {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.replace(" ", "").replace("*", "").replace("-", "+-");
        if let Some(equation) = line.find('=') {
            let (left, rright): (&str, &str) = line.split_at(equation);
            let (_, right): (&str, &str) = rright.split_at(1);

            Ok(Order(left.split("+").filter_map(|term| Term::from_str(term).ok())
                                      .chain(right.split("+")
                                                  .filter_map(|term| Term::from_str(term).ok())
                                                  .map(|term| term.neg()))
                                      .sorted()
                                      .into_iter()
                                      .group_by(|elt: &Term| match elt {
                                            &Term::Determinate(_) => false,
                                            &Term::Indeterminate(_, Degree(power)) => power.rem(2).eq(&1),
                                      })
                                      .into_iter()
                                      .map(|(_, group): (bool, ::itertools::Group<bool, vec::IntoIter<Term>, _>)| group.fold(Term::default(), |acc, term| term.add(acc)))
                                      .filter(|term| term.is_zero().not())
                                      .collect::<Vec<Term>>()))
        } else {
            let mut left = line.split("+").filter_map(|term| Term::from_str(term).ok())
                                          .collect::<Vec<Term>>();

            left.sort();
            Ok(Order(left))
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
