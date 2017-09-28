extern crate computer_v1;

use std::str::FromStr;
use computer_v1::prelude::Order;

#[test]
fn order_from_str() {
    assert_eq!(Order::from_str(""), Ok(Order(vec![])));
    assert_eq!(Order::from_str("+-++++"), Ok(Order(vec![])));
}
