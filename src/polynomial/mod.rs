pub mod order;
pub mod non_zero_constant;
pub mod linear;
pub mod quadratic;

use self::order::term::degree::Degree;
use self::order::term::Term;
use self::order::Order;
use self::non_zero_constant::NonZeroConstant;
use self::linear::Linear;
use self::quadratic::Quadratic;

use ::array::FixedSizeArray;

use ::fmt;
use ::str;
use ::num;
use ::num2;

use ops::{Not, Neg, Mul, Div, Sub, Add, BitAnd};

#[derive(Debug, Clone)]
pub struct Polynomial(Order);

impl Polynomial {
    pub fn is_oob(&self) -> bool {
        self.get_non_zero_constant()
            .is_some()
            .bitand(self.get_linear()
                        .is_some())
            .bitand(self.get_quadratic()
                        .is_some())
            .not()
    }

    pub fn get_non_zero_constant(&self) -> Option<NonZeroConstant> {
        match self.0.as_slice() {
            &[Term::Determinate(_)] | &[] => {
                if self.0.is_empty() {
                    Some(NonZeroConstant::SolubleEverywhere)
                } else {
                    Some(NonZeroConstant::Unsoluble)
                }
            },
            _ => None,
        }
    }

    pub fn get_linear(&self) -> Option<Linear> {
        match self.0.as_slice() {
            &[Term::Indeterminate(a, _),
              Term::Determinate(b)] => {
                match (a, b) {
                    (0.0, 0.0) => Some(Linear::SolubleEverywhere),
                    (0.0, _) => Some(Linear::Unsoluble),
                    (a, b) => Some(Linear::Soluble(b.neg().div(a))),
                }
            }
            _ => None,
        }
    }

    pub fn get_quadratic(&self) -> Option<Quadratic> {
        match self.0.as_slice() {
            &[Term::Indeterminate(a, Degree(2)),
              Term::Indeterminate(b, Degree(1)),
              Term::Determinate(c)] if a != 0f64 => {
                match num2::pow::pow(b, 2).sub(a.mul(c).mul(4.0)) {
                    delta if delta < 0.0 => Some(Quadratic::Negative(a, b)),
                    delta if delta > 0.0 => Some(Quadratic::Positive(delta.sqrt().sub(b).div(a.mul(2.0)),
                                                                     delta.sqrt().add(b).neg().div(a.mul(2.0)))),
                    _ => Some(Quadratic::Null(b.neg().div(a.mul(2.0)))),
                }
            },
            _ => None,
        }
    }
}

impl str::FromStr for Polynomial {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Polynomial(try!(Order::from_str(s))))
    }
}

impl Default for Polynomial {
    fn default() -> Self {
        Polynomial(Order::default())
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Reduced form: {}\n", self.0));
        match self.0.as_slice() {
            &[Term::Indeterminate(a, Degree(2)),
              Term::Indeterminate(b, Degree(1)),
              Term::Determinate(c)] if a != 0f64 => {
                try!(write!(f, "Polynomial degree: 2\n"));
                let delta = num2::pow::pow(b, 2).sub(a.mul(c).mul(4.0));
                try!(write!(f, "Delta: b^2-4ac={b}^2-4*{a}*{c}={d}\n",
                                a = a,
                                b = b,
                                c = c,
                                d = delta));
                match delta {
                    delta if delta < 0.0 => { 
                        write!(f, "Discriminant is strictly negative, the two complex solutions are:\n\
                                   (-b+sqrt(|delta|))/(a*2)=({b}+i*sqrt(|{d}|))/({a}*2)\n\
                                   (-b-sqrt(|delta|))/(a*2)=({b}-i*sqrt(|{d}|))/({a}*2)\n",
                                   a = a,
                                   b = b.neg(),
                                   d = delta)
                    },
                    delta if delta > 0.0 => {
                        write!(f, "Discriminant is strictly positive, the two real solutions are:\n\
                                   (-b+sqrt(delta))/(a*2)=({b}+sqrt({d}))/({a}*2)={x1}\n\
                                   (-b-sqrt(delta))/(a*2)=({b}-sqrt({d}))/({a}*2)={x2}\n",
                                   a = a,
                                   b = b.neg(),
                                   d = delta,
                                   x1 = delta.sqrt().sub(b).div(a.mul(2.0)),
                                   x2 = delta.sqrt().add(b).neg().div(a.mul(2.0)))
                    },
                    _ => {
                        write!(f, "Discriminant is null, the real solution is:\n\
                                   -b/2a={b}/(2*{a})={x}\n",
                                   b = b.neg(),
                                   a = a,
                                   x = b.neg().div(a.mul(2.0)))
                    },
                }
            },
            &[Term::Indeterminate(a, _),
              Term::Determinate(b)] => {
                try!(write!(f, "Polynomial degree: 1\n"));
                match (a, b) {
                    (0.0, 0.0) => write!(f, "All the real number are solutions.\n"), 
                    (0.0, _) => write!(f, "There isn't any solution.\n"), 
                    (a, b) => write!(f, "The solution is:\n\
                                         -a/b=-{b}/{a}={x}\n",
                                          b = b,
                                          a = a,
                                          x = b.neg().div(a)),
                }
            },
            &[Term::Determinate(_)] | &[] => {
                try!(write!(f, "Polynomial degree: 0\n"));
                if self.0.is_empty() {
                    write!(f, "All the real number are solutions.\n")
                } else {
                    write!(f, "There isn't any solution.\n")
                }
            },
            _ => write!(f, "Out of supported polynomes.\n"),
        }
    }
}
