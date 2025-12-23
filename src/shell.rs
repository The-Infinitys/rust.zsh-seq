use crate::TermSequence;
use crate::colors::NamedColor;
use dirs::home_dir;
use std::env;
use std::fmt::Write;
use users::get_current_username;

pub fn print(sequences: &[TermSequence]) -> String {
    let mut result = String::new();

    for sequence in sequences {
        match sequence {
            TermSequence::Percent => result.push('%'),
            TermSequence::BoldStart => result.push_str("\x1b[1m"),
            TermSequence::BoldEnd => result.push_str("\x1b[22m"), // 22は太字解除
            TermSequence::UnderlineStart => result.push_str("\x1b[4m"),
            TermSequence::UnderlineEnd => result.push_str("\x1b[24m"),
            TermSequence::StandoutStart => result.push_str("\x1b[7m"), // 反転
            TermSequence::StandoutEnd => result.push_str("\x1b[27m"),
            TermSequence::StrikethroughStart => result.push_str("\x1b[9m"),
            TermSequence::StrikethroughEnd => result.push_str("\x1b[29m"),
            TermSequence::OverlineStart => result.push_str("\x1b[53m"),
            TermSequence::OverlineEnd => result.push_str("\x1b[55m"),
            TermSequence::BlinkStart => result.push_str("\x1b[5m"),
            TermSequence::BlinkEnd => result.push_str("\x1b[25m"),

            // 色（Foreground）
            TermSequence::ForegroundColor(color) => match color {
                NamedColor::FullColor((r, g, b)) => {
                    write!(result, "\x1b[38;2;{};{};{}m", r, g, b).unwrap();
                }
                _ => {
                    // to_term_string() が 0-255 の数値を返すと想定
                    write!(result, "\x1b[38;5;{}m", color.to_term_string()).unwrap();
                }
            },
            TermSequence::ForegroundColorEnd => result.push_str("\x1b[39m"), // デフォルト色に戻す

            // 色（Background）
            TermSequence::BackgroundColor(color) => match color {
                NamedColor::FullColor((r, g, b)) => {
                    write!(result, "\x1b[48;2;{};{};{}m", r, g, b).unwrap();
                }
                _ => {
                    write!(result, "\x1b[48;5;{}m", color.to_term_string()).unwrap();
                }
            },
            TermSequence::BackgroundColorEnd => result.push_str("\x1b[49m"),

            TermSequence::ResetStyles => result.push_str("\x1b[0m"),

            // システム情報（Rust側で文字列として展開）
            TermSequence::Username => {
                let name = get_current_username()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "unknown".to_string());
                result.push_str(&name);
            }

            TermSequence::HostnameShort => {
                // ホスト名は外部コマンドやlibcが必要なため、簡易的にenvから取得を試みるか、
                // 本来は hostname クレート等を使うのが望ましいです
                let host = env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string());
                result.push_str(host.split('.').next().unwrap_or("localhost"));
            }

            TermSequence::CurrentDirectoryFull => {
                let dir = env::current_dir()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_else(|_| "/".to_string());
                result.push_str(&dir);
            }

            TermSequence::CurrentDirectoryTilde => {
                let current_dir = env::current_dir()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_else(|_| "/".to_string());

                if let Some(home) = home_dir() {
                    let home_str = home.to_string_lossy();
                    if current_dir.starts_with(home_str.as_ref()) {
                        result.push('~');
                        result.push_str(&current_dir[home_str.len()..]);
                    } else {
                        result.push_str(&current_dir);
                    }
                } else {
                    result.push_str(&current_dir);
                }
            }

            TermSequence::PrivilegedIndicator => {
                // 実行ユーザーの権限に応じて記号を変える例
                if users::get_current_uid() == 0 {
                    result.push('#');
                } else {
                    result.push('$');
                }
            }

            TermSequence::Newline => result.push('\n'),
            TermSequence::Literal(s) => result.push_str(s.as_ref()),
        }
    }
    result
}
