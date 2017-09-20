extern crate computer_v1;
extern crate num as num2;

use computer_v1::prelude::Term;
use computer_v1::prelude::Degree;

use num2::CheckedSub;

use std::str::FromStr;
use std::cmp::Ordering;

#[test]
fn term_from_str() {
    assert!(Term::from_str("").is_err());
    assert!(Term::from_str("-").is_err());
    assert!(Term::from_str("+").is_err());
    assert_eq!(Term::from_str("5"), Ok(Term::Determinate(5.0)));
    assert_eq!(Term::from_str("5X^0"), Ok(Term::Determinate(5.0)));
    assert_eq!(Term::from_str("5X^1"), Ok(Term::Indeterminate(5.0, Degree(1))));
    assert_eq!(Term::from_str("5X^-1"), Ok(Term::Indeterminate(5.0, Degree(-1))));
    assert_eq!(Term::from_str("+5"), Ok(Term::Determinate(5.0)));
    assert_eq!(Term::from_str("+5X^0"), Ok(Term::Determinate(5.0)));
    assert_eq!(Term::from_str("+5X^1"), Ok(Term::Indeterminate(5.0, Degree(1))));
    assert_eq!(Term::from_str("+5X^-1"), Ok(Term::Indeterminate(5.0, Degree(-1))));
    assert_eq!(Term::from_str("-5"), Ok(Term::Determinate(-5.0)));
    assert_eq!(Term::from_str("-5X^0"), Ok(Term::Determinate(-5.0)));
    assert_eq!(Term::from_str("-5X^1"), Ok(Term::Indeterminate(-5.0, Degree(1))));
    assert_eq!(Term::from_str("-5X^-1"), Ok(Term::Indeterminate(-5.0, Degree(-1))));
    assert_eq!(Term::from_str("5.5"), Ok(Term::Determinate(5.5)));
    assert_eq!(Term::from_str("5.5X^0"), Ok(Term::Determinate(5.5)));
    assert_eq!(Term::from_str("5.5X^1"), Ok(Term::Indeterminate(5.5, Degree(1))));
    assert_eq!(Term::from_str("5.5X^-1"), Ok(Term::Indeterminate(5.5, Degree(-1))));
    assert_eq!(Term::from_str("+5.5"), Ok(Term::Determinate(5.5)));
    assert_eq!(Term::from_str("+5.5X^0"), Ok(Term::Determinate(5.5)));
    assert_eq!(Term::from_str("+5.5X^1"), Ok(Term::Indeterminate(5.5, Degree(1))));
    assert_eq!(Term::from_str("+5.5X^-1"), Ok(Term::Indeterminate(5.5, Degree(-1))));
    assert_eq!(Term::from_str("-5.5"), Ok(Term::Determinate(-5.5)));
    assert_eq!(Term::from_str("-5.5X^0"), Ok(Term::Determinate(-5.5)));
    assert_eq!(Term::from_str("-5.5X^1"), Ok(Term::Indeterminate(-5.5, Degree(1))));
    assert_eq!(Term::from_str("-5.5X^-1"), Ok(Term::Indeterminate(-5.5, Degree(-1))));
}

#[test]
fn term_checked_sub() {
    assert_eq!(Term::Determinate(0.0).checked_sub(&Term::Determinate(0.0)), Some(Term::Determinate(0.0)));
    assert_eq!(Term::Determinate(10.0).checked_sub(&Term::Determinate(5.0)), Some(Term::Determinate(5.0)));
    assert_eq!(Term::Indeterminate(5.0, Degree(1)).checked_sub(&Term::Indeterminate(10.0, Degree(1))), Some(Term::Indeterminate(5.0, Degree(1))));
    assert_eq!(Term::Indeterminate(5.0, Degree(1)).checked_sub(&Term::Indeterminate(10.0, Degree(2))), Some(Term::Indeterminate(5.0, Degree(1))));
}

#[test]
fn term_cmp() {
    assert_eq!(Term::Determinate(1.0).cmp(&Term::Determinate(2.0)), Ordering::Greater);
    assert_eq!(Term::Determinate(1.0).cmp(&Term::Determinate(1.0)), Ordering::Equal);
    assert_eq!(Term::Determinate(2.0).cmp(&Term::Determinate(1.0)), Ordering::Less);
    assert_eq!(Term::Determinate(0.0).cmp(&Term::Indeterminate(0.0, Degree(1))), Ordering::Greater);
    assert_eq!(Term::Indeterminate(0.0, Degree(1)).cmp(&Term::Determinate(0.0)), Ordering::Less);
    assert_eq!(Term::Indeterminate(1.0, Degree(1)).cmp(&Term::Indeterminate(2.0, Degree(1))), Ordering::Greater);
    assert_eq!(Term::Indeterminate(1.0, Degree(1)).cmp(&Term::Indeterminate(1.0, Degree(1))), Ordering::Equal);
    assert_eq!(Term::Indeterminate(2.0, Degree(1)).cmp(&Term::Indeterminate(1.0, Degree(1))), Ordering::Less);
    assert_eq!(Term::Indeterminate(0.0, Degree(1)).cmp(&Term::Indeterminate(0.0, Degree(2))), Ordering::Greater);
    assert_eq!(Term::Indeterminate(0.0, Degree(1)).cmp(&Term::Indeterminate(0.0, Degree(1))), Ordering::Equal);
    assert_eq!(Term::Indeterminate(0.0, Degree(2)).cmp(&Term::Indeterminate(0.0, Degree(1))), Ordering::Less);
}

#[test]
fn term_partial_cmp() {
    assert_eq!(Term::Determinate(1.0).partial_cmp(&Term::Determinate(2.0)), Some(Ordering::Greater));
    assert_eq!(Term::Determinate(1.0).partial_cmp(&Term::Determinate(1.0)), Some(Ordering::Equal));
    assert_eq!(Term::Determinate(2.0).partial_cmp(&Term::Determinate(1.0)), Some(Ordering::Less));
    assert_eq!(Term::Determinate(0.0).partial_cmp(&Term::Indeterminate(0.0, Degree(1))), Some(Ordering::Greater));
    assert_eq!(Term::Indeterminate(0.0, Degree(1)).partial_cmp(&Term::Determinate(0.0)), Some(Ordering::Less));
    assert_eq!(Term::Indeterminate(1.0, Degree(1)).partial_cmp(&Term::Indeterminate(2.0, Degree(1))), Some(Ordering::Greater));
    assert_eq!(Term::Indeterminate(1.0, Degree(1)).partial_cmp(&Term::Indeterminate(1.0, Degree(1))), Some(Ordering::Equal));
    assert_eq!(Term::Indeterminate(2.0, Degree(1)).partial_cmp(&Term::Indeterminate(1.0, Degree(1))), Some(Ordering::Less));
    assert_eq!(Term::Indeterminate(0.0, Degree(1)).partial_cmp(&Term::Indeterminate(0.0, Degree(2))), Some(Ordering::Greater));
    assert_eq!(Term::Indeterminate(0.0, Degree(1)).partial_cmp(&Term::Indeterminate(0.0, Degree(1))), Some(Ordering::Equal));
    assert_eq!(Term::Indeterminate(0.0, Degree(2)).partial_cmp(&Term::Indeterminate(0.0, Degree(1))), Some(Ordering::Less));
}
