#[derive(Clone, Debug)]
pub enum ScopePropagation {
    None = 0,
    Local = 1,
    Global = 2,
    GlobalAndLocal = 3,
}