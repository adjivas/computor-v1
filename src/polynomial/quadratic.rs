#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Quadratic {
    Negative(f64, f64),
    Positive(f64, f64),
    Null(f64),
}
