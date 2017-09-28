// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/computor-v1
//
// This file may not be copied, modified, or distributed

//! # Computer-v1
//!
//! This library is a polynomial solver.

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

/// The module `polynomial` is a overload of `Order`'s interface.
pub mod polynomial;
/// The module `prelude` is for public.
pub mod prelude;
