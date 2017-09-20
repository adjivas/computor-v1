#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Linear {
    Unsoluble,
    SolubleEverywhere,
    Soluble(f64),
}
