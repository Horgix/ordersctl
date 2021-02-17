use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Status {
    pub confirmed: bool,
    pub paid: bool,
    pub shipped: bool,
    pub received: bool,
}

/// Additional mutation methods for `Option`.
pub trait BoolRepr {
    /// Replace the existing `Some` value with a new one.
    ///
    /// Returns the previous value if it was present, or `None` if no replacement was made.
    fn to_yn(self) -> String;
    fn to_utf8(self) -> String;
    fn to_utf8_heavy(self) -> String;
    fn to_utf8_colored(self) -> String;
}

impl BoolRepr for bool {
    fn to_yn(self) -> String {
        match self {
            true => "Y",
            false => "F",
        }
        .to_string()
    }
    fn to_utf8(self) -> String {
        match self {
            true => "✓",
            false => "✗",
        }
        .to_string()
    }
    fn to_utf8_heavy(self) -> String {
        match self {
            true => "✔",
            false => "✘",
        }
        .to_string()
    }
    fn to_utf8_colored(self) -> String {
        match self {
            true => "✅",
            false => "❌",
        }
        .to_string()
    }
}
