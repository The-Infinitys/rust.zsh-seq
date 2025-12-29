use crate::colors::NamedColor;
use std::env;
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
impl ZshSequence {
    pub fn raw_text(&self) -> String {
        match self {
            ZshSequence::Literal(s) => {
                let mut result = String::new();
                let mut chars = s.chars().peekable();

                while let Some(c) = chars.next() {
                    if c == '%' {
                        match chars.peek() {
                            Some(&'%') => {
                                // %% の場合は、1つの % をリテラルとして残す
                                result.push('%');
                                chars.next(); // 2つ目の % を消費
                            }
                            Some(_) => {
                                // % の後に文字がある場合、その文字も一緒に削除
                                chars.next(); // 次の1文字を読み飛ばす
                            }
                            None => {
                                // 末尾が % 単体で終わっている場合は、その % を削除
                                continue;
                            }
                        }
                    } else {
                        result.push(c);
                    }
                }
                result
            }
            ZshSequence::Percent => "%".to_string(),

            // ユーザー名: $USER を取得
            ZshSequence::Username => users::get_current_username()
                .unwrap()
                .into_string()
                .unwrap(),

            // ホスト名: hostname_short (最初のドットまで)
            ZshSequence::HostnameShort => hostname::get()
                .map(|h| {
                    h.to_string_lossy()
                        .split('.')
                        .next()
                        .unwrap_or("localhost")
                        .to_string()
                })
                .unwrap_or_else(|_| {
                    env::var("HOSTNAME")
                        .map(|h| h.split('.').next().unwrap_or("localhost").to_string())
                        .unwrap_or_else(|_| "localhost".to_string())
                }),

            // カレントディレクトリ (Tildeあり)
            ZshSequence::CurrentDirectoryTilde => {
                let pwd = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                let home = env::var("HOME").map(std::path::PathBuf::from).ok();

                if let Some(h) = home
                    && let Ok(stripped) = pwd.strip_prefix(h)
                {
                    return format!("~/{}", stripped.display())
                        .trim_end_matches('/')
                        .to_string();
                }
                pwd.display().to_string().trim_end_matches('/').to_string()
            }

            // カレントディレクトリ (Full)
            ZshSequence::CurrentDirectoryFull => env::current_dir()
                .map(|p| p.display().to_string().trim_end_matches('/').to_string())
                .unwrap_or_else(|_| "/".to_string()),

            // 特権インジケータ
            ZshSequence::PrivilegedIndicator => {
                // UIDが0なら #、それ以外は % (Unix系前提)
                if users::get_current_uid() == 0 {
                    "#"
                } else {
                    "%"
                }
                .to_string()
            }

            // 改行
            ZshSequence::Newline => "\n".to_string(),

            // スタイル・色関連はテキストとしては「空」
            ZshSequence::BoldStart
            | ZshSequence::BoldEnd
            | ZshSequence::UnderlineStart
            | ZshSequence::UnderlineEnd
            | ZshSequence::StandoutStart
            | ZshSequence::StandoutEnd
            | ZshSequence::ForegroundColor(_)
            | ZshSequence::ForegroundColorEnd
            | ZshSequence::BackgroundColor(_)
            | ZshSequence::BackgroundColorEnd
            | ZshSequence::ResetStyles => String::new(),
        }
    }
}
impl std::fmt::Display for ZshSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZshSequence::Percent => write!(f, "%%"),
            ZshSequence::BoldStart => write!(f, "%B"),
            ZshSequence::BoldEnd => write!(f, "%b"),
            ZshSequence::UnderlineStart => write!(f, "%U"),
            ZshSequence::UnderlineEnd => write!(f, "%u"),
            ZshSequence::StandoutStart => write!(f, "%S"),
            ZshSequence::StandoutEnd => write!(f, "%s"),
            ZshSequence::ForegroundColor(color) => match color {
                NamedColor::FullColor((r, g, b)) => write!(f, "%{{\x1b[38;2;{};{};{}m%}}", r, g, b),
                _ => write!(f, "%F{{{}}}", color.to_zsh_string()),
            },
            ZshSequence::ForegroundColorEnd => write!(f, "%f"),
            ZshSequence::BackgroundColor(color) => match color {
                NamedColor::FullColor((r, g, b)) => write!(f, "%{{\x1b[48;2;{};{};{}m%}}", r, g, b),
                _ => write!(f, "%K{{{}}}", color.to_zsh_string()),
            },
            ZshSequence::BackgroundColorEnd => write!(f, "%k"),
            ZshSequence::ResetStyles => write!(f, "%{{\x1b[0m%}}"),
            ZshSequence::Username => write!(f, "%n"),
            ZshSequence::HostnameShort => write!(f, "%m"),
            ZshSequence::CurrentDirectoryFull => write!(f, "%/"), // Or %d
            ZshSequence::CurrentDirectoryTilde => write!(f, "%~"),
            ZshSequence::PrivilegedIndicator => write!(f, "%#"),
            ZshSequence::Newline => writeln!(f),
            ZshSequence::Literal(s) => {
                for c in s.chars() {
                    if c.is_ascii() {
                        // 通常のASCII文字（英数字など）はそのまま出力
                        write!(f, "{}", c)?;
                    } else {
                        // マルチバイト文字（記号・全角文字など）のみ %{%G...%} で囲む
                        // これにより、Zshに「この文字は見た目に関わらず1文字幅である」と教える
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
