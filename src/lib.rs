#![feature(slice_patterns)]
#![feature(advanced_slice_patterns)]
#![feature(fixed_size_array)]
#![feature(exact_size_is_empty)]

extern crate itertools;
extern crate core;
extern crate num as num2;

use core::array;

use std::ops;
use std::str;
use std::fmt;
use std::cmp;
use std::vec;
use std::num;
use std::iter;

pub mod polynomial;
pub mod prelude;
