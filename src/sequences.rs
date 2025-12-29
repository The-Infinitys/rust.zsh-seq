use crate::colors::NamedColor;
use std::env;
use std::fmt;

/// Represents a Zsh prompt sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZshSequence {
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
    /// Start foreground color (using Zsh named colors or 256-color codes)
    ForegroundColor(NamedColor),
    /// Stop foreground color (%f)
    ForegroundColorEnd,
    /// Start background color (using Zsh named colors or 256-color codes)
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

/// ターゲットとするシェルの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellType {
    Zsh,
    Bash,
}

impl ShellType {
    /// 環境変数 SHELL から現在のシェルを判定する
    pub fn from_env() -> Self {
        match env::var("SHELL") {
            Ok(s) if s.contains("bash") => ShellType::Bash,
            _ => ShellType::Zsh, // デフォルトはzsh
        }
    }
}

impl ZshSequence {
    /// Zsh用のプロンプト文字列を生成
    pub fn zsh(&self) -> String {
        match self {
            ZshSequence::Percent => "%%".to_string(),
            ZshSequence::BoldStart => "%B".to_string(),
            ZshSequence::BoldEnd => "%b".to_string(),
            ZshSequence::UnderlineStart => "%U".to_string(),
            ZshSequence::UnderlineEnd => "%u".to_string(),
            ZshSequence::StandoutStart => "%S".to_string(),
            ZshSequence::StandoutEnd => "%s".to_string(),
            ZshSequence::ForegroundColor(color) => match color {
                NamedColor::FullColor((r, g, b)) => format!("%{{\x1b[38;2;{};{};{}m%}}", r, g, b),
                _ => format!("%F{{{}}}", color.to_zsh_string()),
            },
            ZshSequence::ForegroundColorEnd => "%f".to_string(),
            ZshSequence::BackgroundColor(color) => match color {
                NamedColor::FullColor((r, g, b)) => format!("%{{\x1b[48;2;{};{};{}m%}}", r, g, b),
                _ => format!("%K{{{}}}", color.to_zsh_string()),
            },
            ZshSequence::BackgroundColorEnd => "%k".to_string(),
            ZshSequence::ResetStyles => "%{\x1b[0m%}".to_string(),
            ZshSequence::Username => "%n".to_string(),
            ZshSequence::HostnameShort => "%m".to_string(),
            ZshSequence::CurrentDirectoryFull => "%/".to_string(),
            ZshSequence::CurrentDirectoryTilde => "%~".to_string(),
            ZshSequence::PrivilegedIndicator => "%#".to_string(),
            ZshSequence::Newline => "\n".to_string(),
            ZshSequence::Literal(s) => {
                let mut res = String::new();
                for c in s.chars() {
                    if c.is_ascii() {
                        res.push(c);
                    } else {
                        res.push_str(&format!("%{{%G{}%}}", c));
                    }
                }
                res
            }
        }
    }

    /// Bash用のプロンプト文字列を生成
    /// 非表示文字（ANSIエスケープなど）は \x01 と \x02 で囲む必要がある
    pub fn bash(&self) -> String {
        // ヘルパー：制御文字（幅ゼロ文字）をタグで囲む
        let wrap = |content: String| format!("\x01{}\x02", content);

        match self {
            ZshSequence::Percent => "%".to_string(),
            ZshSequence::BoldStart => wrap("\x1b[1m".to_string()),
            ZshSequence::BoldEnd => wrap("\x1b[22m".to_string()),
            ZshSequence::UnderlineStart => wrap("\x1b[4m".to_string()),
            ZshSequence::UnderlineEnd => wrap("\x1b[24m".to_string()),
            ZshSequence::StandoutStart => wrap("\x1b[7m".to_string()),
            ZshSequence::StandoutEnd => wrap("\x1b[27m".to_string()),
            ZshSequence::ForegroundColor(color) => match color {
                NamedColor::FullColor((r, g, b)) => wrap(format!("\x1b[38;2;{};{};{}m", r, g, b)),
                _ => wrap(format!("\x1b[38;5;{}m", color.to_bash_string())), // 256色用メソッドを想定
            },
            ZshSequence::ForegroundColorEnd | ZshSequence::ResetStyles => {
                wrap("\x1b[0m".to_string())
            }
            ZshSequence::BackgroundColor(color) => match color {
                NamedColor::FullColor((r, g, b)) => wrap(format!("\x1b[48;2;{};{};{}m", r, g, b)),
                _ => wrap(format!("\x1b[48;5;{}m", color.to_bash_string())),
            },
            ZshSequence::BackgroundColorEnd => wrap("\x1b[49m".to_string()),
            ZshSequence::Username => "\\u".to_string(),
            ZshSequence::HostnameShort => "\\h".to_string(),
            ZshSequence::CurrentDirectoryFull => "\\w".to_string(), // Bashは基本 \w か \W
            ZshSequence::CurrentDirectoryTilde => "\\w".to_string(),
            ZshSequence::PrivilegedIndicator => "\\$".to_string(),
            ZshSequence::Newline => "\n".to_string(),
            ZshSequence::Literal(s) => s.clone(),
        }
    }
}

impl fmt::Display for ZshSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shell = ShellType::from_env();
        match shell {
            ShellType::Zsh => write!(f, "{}", self.zsh()),
            ShellType::Bash => write!(f, "{}", self.bash()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::NamedColor; // NamedColorをインポート

    #[test]
    fn test_percent_sequence() {
        assert_eq!(ZshSequence::Percent.to_string(), "%%");
    }

    #[test]
    fn test_bold_sequences() {
        assert_eq!(ZshSequence::BoldStart.to_string(), "%B");
        assert_eq!(ZshSequence::BoldEnd.to_string(), "%b");
    }

    #[test]
    fn test_underline_sequences() {
        assert_eq!(ZshSequence::UnderlineStart.to_string(), "%U");
        assert_eq!(ZshSequence::UnderlineEnd.to_string(), "%u");
    }

    #[test]
    fn test_foreground_color_sequence() {
        assert_eq!(
            ZshSequence::ForegroundColor(NamedColor::Red).to_string(),
            "%F{red}"
        );
        assert_eq!(
            ZshSequence::ForegroundColor(NamedColor::Code256(123)).to_string(),
            "%F{123}"
        );
        assert_eq!(ZshSequence::ForegroundColorEnd.to_string(), "%f");
    }

    #[test]
    fn test_background_color_sequence() {
        assert_eq!(
            ZshSequence::BackgroundColor(NamedColor::Blue).to_string(),
            "%K{blue}"
        );
        assert_eq!(
            ZshSequence::BackgroundColor(NamedColor::Code256(200)).to_string(),
            "%K{200}"
        );
        assert_eq!(ZshSequence::BackgroundColorEnd.to_string(), "%k");
    }

    #[test]
    fn test_reset_styles_sequence() {
        assert_eq!(ZshSequence::ResetStyles.to_string(), "%{\x1b[0m%}");
    }

    #[test]
    fn test_username_sequence() {
        assert_eq!(ZshSequence::Username.to_string(), "%n");
    }

    #[test]
    fn test_hostname_short_sequence() {
        assert_eq!(ZshSequence::HostnameShort.to_string(), "%m");
    }

    #[test]
    fn test_current_directory_full_sequence() {
        assert_eq!(ZshSequence::CurrentDirectoryFull.to_string(), "%/");
    }

    #[test]
    fn test_current_directory_tilde_sequence() {
        assert_eq!(ZshSequence::CurrentDirectoryTilde.to_string(), "%~");
    }

    #[test]
    fn test_privileged_indicator_sequence() {
        assert_eq!(ZshSequence::PrivilegedIndicator.to_string(), "%#");
    }

    #[test]
    fn test_literal_sequence() {
        assert_eq!(
            ZshSequence::Literal("hello".to_string()).to_string(),
            "hello"
        );
    }
}
