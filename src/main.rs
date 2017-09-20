extern crate computer_v1;

use computer_v1::prelude::Polynomial;

use std::env;
use std::str::FromStr;

fn main() {
    let merged: String =
        env::args().skip(1)
                   .filter_map(|arg: String| Polynomial::from_str(&arg).ok())
                   .map(|result: Polynomial| format!("{}", result))
                   .collect();
                
    print!("{}", merged);
}
