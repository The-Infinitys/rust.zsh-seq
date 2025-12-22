use std::fmt;

/// Represents a color for Term prompt sequences (named colors or 256-color codes).
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    LightBlack, // Often Dark Gray (256-color code 240)
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite,  // Often Bright White
    Code256(u8), // 0-255
    FullColor((u8, u8, u8)),
}

impl fmt::Display for NamedColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            NamedColor::Black => "Black".to_string(),
            NamedColor::Red => "Red".to_string(),
            NamedColor::Green => "Green".to_string(),
            NamedColor::Yellow => "Yellow".to_string(),
            NamedColor::Blue => "Blue".to_string(),
            NamedColor::Magenta => "Magenta".to_string(),
            NamedColor::Cyan => "Cyan".to_string(),
            NamedColor::White => "White".to_string(),
            NamedColor::LightBlack => "LightBlack".to_string(),
            NamedColor::LightRed => "LightRed".to_string(),
            NamedColor::LightGreen => "LightGreen".to_string(),
            NamedColor::LightYellow => "LightYellow".to_string(),
            NamedColor::LightBlue => "LightBlue".to_string(),
            NamedColor::LightMagenta => "LightMagenta".to_string(),
            NamedColor::LightCyan => "LightCyan".to_string(),
            NamedColor::LightWhite => "LightWhite".to_string(),
            NamedColor::Code256(code) => format!("Code256({})", code),
            NamedColor::FullColor((r, g, b)) => format!("FullColor({},{},{})", r, g, b),
        };
        write!(f, "{}", s)
    }
}

impl NamedColor {
    /// Converts the NamedColor enum to its string representation for Term.
    pub fn to_term_string(&self) -> String {
        match self {
            NamedColor::Black => "black".to_string(),
            NamedColor::Red => "red".to_string(),
            NamedColor::Green => "green".to_string(),
            NamedColor::Yellow => "yellow".to_string(),
            NamedColor::Blue => "blue".to_string(),
            NamedColor::Magenta => "magenta".to_string(),
            NamedColor::Cyan => "cyan".to_string(),
            NamedColor::White => "white".to_string(),
            NamedColor::LightBlack => "240".to_string(),
            NamedColor::LightRed => "lightred".to_string(),
            NamedColor::LightGreen => "lightgreen".to_string(),
            NamedColor::LightYellow => "lightyellow".to_string(),
            NamedColor::LightBlue => "lightblue".to_string(),
            NamedColor::LightMagenta => "lightmagenta".to_string(),
            NamedColor::LightCyan => "lightcyan".to_string(),
            NamedColor::LightWhite => "white".to_string(),
            NamedColor::Code256(code) => code.to_string(),
            NamedColor::FullColor((_, _, _)) => {
                // NamedColor::FullColorはエスケープシーケンスとして処理されるため、
                // このメソッドで直接Term文字列に変換されるべきではない。
                // したがって、ここに到達することは論理エラーを示す。
                panic!(
                    "NamedColor::FullColor should be handled as an escape sequence, not converted to a Term string directly."
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {}
