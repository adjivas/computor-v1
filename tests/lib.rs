extern crate computer_v1;

use std::str::FromStr;
use computer_v1::prelude::{Polynomial, Order, Term, Degree, NonZeroConstant, Linear, Quadratic};

#[test]
fn subject() {
    assert_eq!(Order::from_str("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0").unwrap_or_default(),
               Order(vec![Term::Indeterminate(-9.3, Degree(2)), Term::Indeterminate(4.0, Degree(1)), Term::Determinate(4.0)])); 
    assert_eq!(Polynomial::from_str("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0").unwrap_or_default().get_quadratic(),
               Some(Quadratic::Positive(-0.47513146390886934, 0.9052389907905898)));
    assert_eq!(Order::from_str("5 * X^0 + 4 * X^1 = 4 * X^0").unwrap_or_default(),
               Order(vec![Term::Indeterminate(4.0, Degree(1)), Term::Determinate(1.0)])); 
    assert_eq!(Polynomial::from_str("5 * X^0 + 4 * X^1 = 4 * X^0").unwrap_or_default().get_linear(),
               Some(Linear::Soluble(-0.25)));
    assert_eq!(Order::from_str("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0").unwrap_or_default(),
               Order(vec![Term::Indeterminate(-5.6, Degree(3)), Term::Indeterminate(-6.0, Degree(1)), Term::Determinate(5.0)])); 
    assert!(Polynomial::from_str("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0").unwrap_or_default().is_oob());
}

#[test]
fn correction() {
    assert_eq!(Polynomial::from_str("5 * X^0 = 5 * X^0").unwrap_or_default().get_non_zero_constant(),
               Some(NonZeroConstant::SolubleEverywhere));
    assert_eq!(Polynomial::from_str("3 * X^0 = 8 * X^0").unwrap_or_default().get_non_zero_constant(),
               Some(NonZeroConstant::Unsoluble));
    assert_eq!(Polynomial::from_str("5 * X^0 = 4 * X^0 + 7 * X^1").unwrap_or_default().get_linear(),
               Some(Linear::Soluble(0.14285714285714285)));
    assert_eq!(Polynomial::from_str("5 * X^0 + 13 * X^1 + 3 * X^2 = 1 *X^0 + 1 * X^1").unwrap_or_default().get_quadratic(),
               Some(Quadratic::Positive(-0.3670068381445481, -3.632993161855452)));
    assert_eq!(Polynomial::from_str("6 * X^0 + 11 * X^1 + 5 * X^2 = 1 * X^0 + 1 * X^1").unwrap_or_default().get_quadratic(),
               Some(Quadratic::Null(-1.0)));
    assert_eq!(Polynomial::from_str("5 * X^0 + 3 * X^1 + 3 * X^2 = 1 * X^0 + 0 * X^1").unwrap_or_default().get_quadratic(),
               Some(Quadratic::Negative(3.0, 3.0)));
}
