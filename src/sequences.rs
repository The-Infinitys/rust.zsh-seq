use crate::builder::TermType;
use crate::colors::NamedColor;
#[cfg(test)]
mod tests;
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
    /// Start strikethrough mode
    StrikethroughStart,
    /// Stop strikethrough mode
    StrikethroughEnd,
    /// Start overline mode
    OverlineStart,
    /// Stop overline mode
    OverlineEnd,
    /// Start blinking mode
    BlinkStart,
    /// Stop blinking mode
    BlinkEnd,
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

impl TermSequence {
    pub fn to_shell_string(&self, term_type: TermType) -> String {
        match term_type {
            TermType::Zsh => match self {
                TermSequence::Percent => "%%".to_string(), // Zshでリテラルの%は %%
                TermSequence::BoldStart => "%B".to_string(),
                TermSequence::BoldEnd => "%b".to_string(),
                TermSequence::UnderlineStart => "%U".to_string(),
                TermSequence::UnderlineEnd => "%u".to_string(),
                TermSequence::StandoutStart => "%S".to_string(),
                TermSequence::StandoutEnd => "%s".to_string(),
                // SGRエスケープは %{ ... %} で囲む
                TermSequence::StrikethroughStart => "%{\x1b[9m%}".to_string(),
                TermSequence::StrikethroughEnd => "%{\x1b[29m%}".to_string(),
                TermSequence::OverlineStart => "%{\x1b[53m%}".to_string(),
                TermSequence::OverlineEnd => "%{\x1b[55m%}".to_string(),
                TermSequence::BlinkStart => "%{\x1b[5m%}".to_string(),
                TermSequence::BlinkEnd => "%{\x1b[25m%}".to_string(),
                TermSequence::ForegroundColor(color) => match color {
                    NamedColor::FullColor((r, g, b)) => {
                        format!("%{{\x1b[38;2;{};{};{}m%}}", r, g, b)
                    }
                    _ => format!("%F{{{}}}", color.to_term_string()),
                },
                TermSequence::ForegroundColorEnd => "%f".to_string(),
                TermSequence::BackgroundColor(color) => match color {
                    NamedColor::FullColor((r, g, b)) => {
                        format!("%{{\x1b[48;2;{};{};{}m%}}", r, g, b)
                    }
                    _ => format!("%K{{{}}}", color.to_term_string()),
                },
                TermSequence::BackgroundColorEnd => "%k".to_string(),
                TermSequence::ResetStyles => "%{\x1b[0m%}".to_string(),
                TermSequence::Username => "%n".to_string(),
                TermSequence::HostnameShort => "%m".to_string(),
                TermSequence::CurrentDirectoryFull => "%/".to_string(),
                TermSequence::CurrentDirectoryTilde => "%~".to_string(),
                TermSequence::PrivilegedIndicator => "%#".to_string(),
                TermSequence::Newline => "\n".to_string(),
                TermSequence::Literal(s) => s.replace('%', "%%"), // リテラル内の%をエスケープ
            },

            TermType::Bash => match self {
                TermSequence::Percent => "%%".to_string(),
                // Bashは \[\e[ ... m\] を使用。特定の属性解除コードを使用し、他のスタイルを壊さないようにする
                TermSequence::BoldStart => r"\[\e[1m\]".to_string(),
                TermSequence::BoldEnd => r"\[\e[22m\]".to_string(),
                TermSequence::UnderlineStart => r"\[\e[4m\]".to_string(),
                TermSequence::UnderlineEnd => r"\[\e[24m\]".to_string(),
                TermSequence::StandoutStart => r"\[\e[7m\]".to_string(),
                TermSequence::StandoutEnd => r"\[\e[27m\]".to_string(),
                TermSequence::StrikethroughStart => r"\[\e[9m\]".to_string(),
                TermSequence::StrikethroughEnd => r"\[\e[29m\]".to_string(),
                TermSequence::OverlineStart => r"\[\e[53m\]".to_string(),
                TermSequence::OverlineEnd => r"\[\e[55m\]".to_string(),
                TermSequence::BlinkStart => r"\[\e[5m\]".to_string(),
                TermSequence::BlinkEnd => r"\[\e[25m\]".to_string(),
                TermSequence::ForegroundColor(color) => match color {
                    NamedColor::FullColor((r, g, b)) => format!(r"\[\e[38;2;{};{};{}m\]", r, g, b),
                    _ => format!(r"\[\e[38;5;{}m\]", color.to_term_string()),
                },
                TermSequence::ForegroundColorEnd => r"\[\e[39m\]".to_string(),
                TermSequence::BackgroundColor(color) => match color {
                    NamedColor::FullColor((r, g, b)) => format!(r"\[\e[48;2;{};{};{}m\]", r, g, b),
                    _ => format!(r"\[\e[48;5;{}m\]", color.to_term_string()),
                },
                TermSequence::BackgroundColorEnd => r"\[\e[49m\]".to_string(),
                TermSequence::ResetStyles => r"\[\e[0m\]".to_string(),
                TermSequence::Username => r"\u".to_string(),
                TermSequence::HostnameShort => r"\h".to_string(),
                TermSequence::CurrentDirectoryFull => r"\w".to_string(),
                TermSequence::CurrentDirectoryTilde => r"\w".to_string(),
                TermSequence::PrivilegedIndicator => r"\$".to_string(),
                TermSequence::Newline => r"\n".to_string(),
                TermSequence::Literal(s) => s.clone(),
            },

            TermType::Tmux | TermType::Pwsh | TermType::Fish => {
                // これらのシェルは共通して、\[ \] のようなパディング囲みなしで
                // 純粋な ANSI エスケープを解釈できる
                match self {
                    TermSequence::Percent => "%".to_string(),
                    TermSequence::BoldStart => "\x1b[1m".to_string(),
                    TermSequence::BoldEnd => "\x1b[22m".to_string(),
                    TermSequence::UnderlineStart => "\x1b[4m".to_string(),
                    TermSequence::UnderlineEnd => "\x1b[24m".to_string(),
                    TermSequence::StandoutStart => "\x1b[7m".to_string(),
                    TermSequence::StandoutEnd => "\x1b[27m".to_string(),
                    TermSequence::StrikethroughStart => "\x1b[9m".to_string(),
                    TermSequence::StrikethroughEnd => "\x1b[29m".to_string(),
                    TermSequence::OverlineStart => "\x1b[53m".to_string(),
                    TermSequence::OverlineEnd => "\x1b[55m".to_string(),
                    TermSequence::BlinkStart => "\x1b[5m".to_string(),
                    TermSequence::BlinkEnd => "\x1b[25m".to_string(),
                    TermSequence::ForegroundColor(color) => match color {
                        NamedColor::FullColor((r, g, b)) => format!("\x1b[38;2;{};{};{}m", r, g, b),
                        _ => format!("\x1b[38;5;{}m", color.to_term_string()),
                    },
                    TermSequence::ForegroundColorEnd => "\x1b[39m".to_string(),
                    TermSequence::BackgroundColor(color) => match color {
                        NamedColor::FullColor((r, g, b)) => format!("\x1b[48;2;{};{};{}m", r, g, b),
                        _ => format!("\x1b[48;5;{}m", color.to_term_string()),
                    },
                    TermSequence::BackgroundColorEnd => "\x1b[49m".to_string(),
                    TermSequence::ResetStyles => "\x1b[0m".to_string(),

                    // シェルごとの動的プレースホルダ
                    TermSequence::Username => match term_type {
                        TermType::Tmux => "#(whoami)".to_string(),
                        TermType::Pwsh => "$env:USERNAME".to_string(),
                        _ => "(whoami)".to_string(),
                    },
                    TermSequence::HostnameShort => match term_type {
                        TermType::Tmux => "#H".to_string(),
                        TermType::Pwsh => "$env:COMPUTERNAME".to_string(),
                        _ => "(hostname -s)".to_string(),
                    },
                    TermSequence::CurrentDirectoryFull => match term_type {
                        TermType::Tmux => "#{pane_current_path}".to_string(),
                        TermType::Pwsh => "$PWD".to_string(),
                        _ => "(pwd)".to_string(),
                    },
                    TermSequence::CurrentDirectoryTilde => match term_type {
                        TermType::Tmux => "#{pane_current_path}".to_string(), // Tmuxはパス置換が複雑なため
                        TermType::Pwsh => "$(Split-Path $PWD -Leaf)".to_string(), // 簡易例
                        _ => "(prompt_pwd)".to_string(),
                    },
                    TermSequence::PrivilegedIndicator => "$".to_string(),
                    TermSequence::Newline => "\n".to_string(),
                    TermSequence::Literal(s) => s.clone(),
                }
            }
        }
    }
}
