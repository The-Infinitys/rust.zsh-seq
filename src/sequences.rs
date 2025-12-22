use crate::colors::NamedColor;

/// Represents a Term prompt sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TermSequence {
    /// Literal '%'
    Percent,
    /// Start boldface mode
    BoldStart,
    /// Stop boldface mode
    BoldEnd,
    /// Start underline mode
    UnderlineStart,
    /// Stop underline mode
    UnderlineEnd,
    /// Start standout mode
    StandoutStart,
    /// Stop standout mode
    StandoutEnd,
    /// Start foreground color (using Term named colors or 256-color codes)
    ForegroundColor(NamedColor),
    /// Stop foreground color (%f)
    ForegroundColorEnd,
    /// Start background color (using Term named colors or 256-color codes)
    BackgroundColor(NamedColor),
    /// Stop background color (%k)
    BackgroundColorEnd,
    /// Reset all styles and colors - generates %{\x1b[0m%}
    ResetStyles,
    /// Username
    Username,
    /// Hostname (short)
    HostnameShort,
    /// Current working directory (full path)
    CurrentDirectoryFull,
    /// Current working directory (with tilde expansion)
    CurrentDirectoryTilde,
    /// # if privileged, % if not
    PrivilegedIndicator,
    /// Newline (Physical line break)
    Newline,
    /// Custom string that can be inserted directly.
    Literal(String),
}

impl std::fmt::Display for TermSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TermSequence::Percent => write!(f, "%%%"),
            TermSequence::BoldStart => write!(f, "%B"),
            TermSequence::BoldEnd => write!(f, "%b"),
            TermSequence::UnderlineStart => write!(f, "%U"),
            TermSequence::UnderlineEnd => write!(f, "%u"),
            TermSequence::StandoutStart => write!(f, "%S"),
            TermSequence::StandoutEnd => write!(f, "%s"),
            TermSequence::ForegroundColor(color) => match color {
                NamedColor::FullColor((r, g, b)) => write!(f, "%{{\x1b[38;2;{};{};{}m%}}", r, g, b),
                _ => write!(f, "%F{{{}}}", color.to_term_string()),
            },
            TermSequence::ForegroundColorEnd => write!(f, "%f"),
            TermSequence::BackgroundColor(color) => match color {
                NamedColor::FullColor((r, g, b)) => write!(f, "%{{\x1b[48;2;{};{};{}m%}}", r, g, b),
                _ => write!(f, "%K{{{}}}", color.to_term_string()),
            },
            TermSequence::BackgroundColorEnd => write!(f, "%k"),
            TermSequence::ResetStyles => write!(f, "%{{\x1b[0m%}}"),
            TermSequence::Username => write!(f, "%n"),
            TermSequence::HostnameShort => write!(f, "%m"),
            TermSequence::CurrentDirectoryFull => write!(f, "%/"), // Or %d
            TermSequence::CurrentDirectoryTilde => write!(f, "%~"),
            TermSequence::PrivilegedIndicator => write!(f, "%#"),
            TermSequence::Newline => writeln!(f),
            TermSequence::Literal(s) => {
                for c in s.chars() {
                    if c.is_ascii() {
                        // 通常のASCII文字（英数字など）はそのまま出力
                        write!(f, "{}", c)?;
                    } else {
                        // マルチバイト文字（記号・全角文字など）のみ %{%G...%} で囲む
                        // これにより、Termに「この文字は見た目に関わらず1文字幅である」と教える
                        write!(f, "%{{%G{}%}}", c)?;
                    }
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::NamedColor; // NamedColorをインポート

    #[test]
    fn test_percent_sequence() {
        assert_eq!(TermSequence::Percent.to_string(), "%%%");
    }

    #[test]
    fn test_bold_sequences() {
        assert_eq!(TermSequence::BoldStart.to_string(), "%B");
        assert_eq!(TermSequence::BoldEnd.to_string(), "%b");
    }

    #[test]
    fn test_underline_sequences() {
        assert_eq!(TermSequence::UnderlineStart.to_string(), "%U");
        assert_eq!(TermSequence::UnderlineEnd.to_string(), "%u");
    }

    #[test]
    fn test_foreground_color_sequence() {
        assert_eq!(
            TermSequence::ForegroundColor(NamedColor::Red).to_string(),
            "%F{red}"
        );
        assert_eq!(
            TermSequence::ForegroundColor(NamedColor::Code256(123)).to_string(),
            "%F{123}"
        );
        assert_eq!(TermSequence::ForegroundColorEnd.to_string(), "%f");
    }

    #[test]
    fn test_background_color_sequence() {
        assert_eq!(
            TermSequence::BackgroundColor(NamedColor::Blue).to_string(),
            "%K{blue}"
        );
        assert_eq!(
            TermSequence::BackgroundColor(NamedColor::Code256(200)).to_string(),
            "%K{200}"
        );
        assert_eq!(TermSequence::BackgroundColorEnd.to_string(), "%k");
    }

    #[test]
    fn test_reset_styles_sequence() {
        assert_eq!(TermSequence::ResetStyles.to_string(), "%{\x1b[0m%}");
    }

    #[test]
    fn test_username_sequence() {
        assert_eq!(TermSequence::Username.to_string(), "%n");
    }

    #[test]
    fn test_hostname_short_sequence() {
        assert_eq!(TermSequence::HostnameShort.to_string(), "%m");
    }

    #[test]
    fn test_current_directory_full_sequence() {
        assert_eq!(TermSequence::CurrentDirectoryFull.to_string(), "%/");
    }

    #[test]
    fn test_current_directory_tilde_sequence() {
        assert_eq!(TermSequence::CurrentDirectoryTilde.to_string(), "%~");
    }

    #[test]
    fn test_privileged_indicator_sequence() {
        assert_eq!(TermSequence::PrivilegedIndicator.to_string(), "%#");
    }

    #[test]
    fn test_literal_sequence() {
        assert_eq!(
            TermSequence::Literal("hello".to_string()).to_string(),
            "hello"
        );
    }
}
