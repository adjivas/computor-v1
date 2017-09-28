// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/computor-v1
//
// This file may not be copied, modified, or distributed

//! This module is a quadratic definition.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Quadratic {
    Negative(f64, f64),
    Positive(f64, f64),
    Null(f64),
}
