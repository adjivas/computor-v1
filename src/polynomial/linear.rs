// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/computor-v1
//
// This file may not be copied, modified, or distributed

//! This module is a linear definition.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Linear {
    Unsoluble,
    SolubleEverywhere,
    Soluble(f64),
}

