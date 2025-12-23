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
                TermSequence::Percent => "%%%".to_string(),
                TermSequence::BoldStart => "%B".to_string(),
                TermSequence::BoldEnd => "%b".to_string(),
                TermSequence::UnderlineStart => "%U".to_string(),
                TermSequence::UnderlineEnd => "%u".to_string(),
                TermSequence::StandoutStart => "%S".to_string(),
                TermSequence::StandoutEnd => "%s".to_string(),
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
                TermSequence::Literal(s) => {
                    let mut result = String::new();
                    for c in s.chars() {
                        if c.is_ascii() {
                            result.push(c);
                        } else {
                            result.push_str(&format!("%{{%G{}%}}", c));
                        }
                    }
                    result
                }
            },
            TermType::Bash => {
                match self {
                    TermSequence::Percent => "%%".to_string(), // Bash: %% for single %
                    TermSequence::BoldStart => "\\[\\e[1m\\]".to_string(),
                    TermSequence::BoldEnd => "\\[\\e[22m\\]".to_string(), // Or \e[0m
                    TermSequence::UnderlineStart => "\\[\\e[4m\\]".to_string(),
                    TermSequence::UnderlineEnd => "\\[\\e[24m\\]".to_string(), // Or \e[0m
                    TermSequence::StandoutStart => "\\[\\e[7m\\]".to_string(), // Reverse video
                    TermSequence::StandoutEnd => "\\[\\e[27m\\]".to_string(),  // Or \e[0m
                    TermSequence::StrikethroughStart => "\\[\\e[9m\\]".to_string(),
                    TermSequence::StrikethroughEnd => "\\[\\e[29m\\]".to_string(),
                    TermSequence::OverlineStart => "\\[\\e[53m\\]".to_string(),
                    TermSequence::OverlineEnd => "\\[\\e[55m\\]".to_string(),
                    TermSequence::BlinkStart => "\\[\\e[5m\\]".to_string(),
                    TermSequence::BlinkEnd => "\\[\\e[25m\\]".to_string(),
                    TermSequence::ForegroundColor(color) => match color {
                        NamedColor::FullColor((r, g, b)) => {
                            format!("\\[\\e[38;2;{};{};{}m\\]", r, g, b)
                        }
                        NamedColor::Black => "\\[\\e[30m\\]".to_string(),
                        NamedColor::Red => "\\[\\e[31m\\]".to_string(),
                        NamedColor::Green => "\\[\\e[32m\\]".to_string(),
                        NamedColor::Yellow => "\\[\\e[33m\\]".to_string(),
                        NamedColor::Blue => "\\[\\e[34m\\]".to_string(),
                        NamedColor::Magenta => "\\[\\e[35m\\]".to_string(),
                        NamedColor::Cyan => "\\[\\e[36m\\]".to_string(),
                        NamedColor::White => "\\[\\e[37m\\]".to_string(),
                        NamedColor::LightBlack => "\\[\\e[90m\\]".to_string(), // Bright Black / Dark Gray
                        NamedColor::LightRed => "\\[\\e[91m\\]".to_string(),
                        NamedColor::LightGreen => "\\[\\e[92m\\]".to_string(),
                        NamedColor::LightYellow => "\\[\\e[93m\\]".to_string(),
                        NamedColor::LightBlue => "\\[\\e[94m\\]".to_string(),
                        NamedColor::LightMagenta => "\\[\\e[95m\\]".to_string(),
                        NamedColor::LightCyan => "\\[\\e[96m\\]".to_string(),
                        NamedColor::LightWhite => "\\[\\e[97m\\]".to_string(),
                        NamedColor::Code256(code) => format!("\\[\\e[38;5;{}m\\]", code),
                    },
                    TermSequence::ForegroundColorEnd => "\\[\\e[39m\\]".to_string(), // Reset foreground
                    TermSequence::BackgroundColor(color) => match color {
                        NamedColor::FullColor((r, g, b)) => {
                            format!("\\[\\e[48;2;{};{};{}m\\]", r, g, b)
                        }
                        NamedColor::Black => "\\[\\e[40m\\]".to_string(),
                        NamedColor::Red => "\\[\\e[41m\\]".to_string(),
                        NamedColor::Green => "\\[\\e[42m\\]".to_string(),
                        NamedColor::Yellow => "\\[\\e[43m\\]".to_string(),
                        NamedColor::Blue => "\\[\\e[44m\\]".to_string(),
                        NamedColor::Magenta => "\\[\\e[45m\\]".to_string(),
                        NamedColor::Cyan => "\\[\\e[46m\\]".to_string(),
                        NamedColor::White => "\\[\\e[47m\\]".to_string(),
                        NamedColor::LightBlack => "\\[\\e[100m\\]".to_string(), // Bright Black / Dark Gray
                        NamedColor::LightRed => "\\[\\e[101m\\]".to_string(),
                        NamedColor::LightGreen => "\\[\\e[102m\\]".to_string(),
                        NamedColor::LightYellow => "\\[\\e[103m\\]".to_string(),
                        NamedColor::LightBlue => "\\[\\e[104m\\]".to_string(),
                        NamedColor::LightMagenta => "\\[\\e[105m\\]".to_string(),
                        NamedColor::LightCyan => "\\[\\e[106m\\]".to_string(),
                        NamedColor::LightWhite => "\\[\\e[107m\\]".to_string(),
                        NamedColor::Code256(code) => format!("\\[\\e[48;5;{}m\\]", code),
                    },
                    TermSequence::BackgroundColorEnd => "\\[\\e[49m\\]".to_string(), // Reset background
                    TermSequence::ResetStyles => "\\[\\e[0m\\]".to_string(),
                    TermSequence::Username => "\\u".to_string(),
                    TermSequence::HostnameShort => "\\h".to_string(),
                    TermSequence::CurrentDirectoryFull => "\\w".to_string(),
                    TermSequence::CurrentDirectoryTilde => "\\w".to_string(), // \w in Bash usually tilde-expands
                    TermSequence::PrivilegedIndicator => "\\$".to_string(),
                    TermSequence::Newline => "\\n".to_string(),
                    TermSequence::Literal(s) => s.clone(),
                }
            }
            TermType::Tmux => {
                // Tmux generally passes through ANSI escape sequences, so we can use Bash-like sequences without the \[\e ... \]] wrappers.
                // However, dynamic information like username, hostname, etc., needs to be obtained from the underlying shell or tmux itself.
                // For simplicity, we'll use similar escape sequences as Bash but without the \[ and \] wrappers for non-printable characters.
                // Dynamic prompt elements should ideally be handled by the builder for Tmux, or directly by tmux's status line commands.
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
                        NamedColor::Black => "\x1b[30m".to_string(),
                        NamedColor::Red => "\x1b[31m".to_string(),
                        NamedColor::Green => "\x1b[32m".to_string(),
                        NamedColor::Yellow => "\x1b[33m".to_string(),
                        NamedColor::Blue => "\x1b[34m".to_string(),
                        NamedColor::Magenta => "\x1b[35m".to_string(),
                        NamedColor::Cyan => "\x1b[36m".to_string(),
                        NamedColor::White => "\x1b[37m".to_string(),
                        NamedColor::LightBlack => "\x1b[90m".to_string(),
                        NamedColor::LightRed => "\x1b[91m".to_string(),
                        NamedColor::LightGreen => "\x1b[92m".to_string(),
                        NamedColor::LightYellow => "\x1b[93m".to_string(),
                        NamedColor::LightBlue => "\x1b[94m".to_string(),
                        NamedColor::LightMagenta => "\x1b[95m".to_string(),
                        NamedColor::LightCyan => "\x1b[96m".to_string(),
                        NamedColor::LightWhite => "\x1b[97m".to_string(),
                        NamedColor::Code256(code) => format!("\x1b[38;5;{}m", code),
                    },
                    TermSequence::ForegroundColorEnd => "\x1b[39m".to_string(),
                    TermSequence::BackgroundColor(color) => match color {
                        NamedColor::FullColor((r, g, b)) => format!("\x1b[48;2;{};{};{}m", r, g, b),
                        NamedColor::Black => "\x1b[40m".to_string(),
                        NamedColor::Red => "\x1b[41m".to_string(),
                        NamedColor::Green => "\x1b[42m".to_string(),
                        NamedColor::Yellow => "\x1b[43m".to_string(),
                        NamedColor::Blue => "\x1b[44m".to_string(),
                        NamedColor::Magenta => "\x1b[45m".to_string(),
                        NamedColor::Cyan => "\x1b[46m".to_string(),
                        NamedColor::White => "\x1b[47m".to_string(),
                        NamedColor::LightBlack => "\x1b[100m".to_string(),
                        NamedColor::LightRed => "\x1b[101m".to_string(),
                        NamedColor::LightGreen => "\x1b[102m".to_string(),
                        NamedColor::LightYellow => "\x1b[103m".to_string(),
                        NamedColor::LightBlue => "\x1b[104m".to_string(),
                        NamedColor::LightMagenta => "\x1b[105m".to_string(),
                        NamedColor::LightCyan => "\x1b[106m".to_string(),
                        NamedColor::LightWhite => "\x1b[107m".to_string(),
                        NamedColor::Code256(code) => format!("\x1b[48;5;{}m", code),
                    },
                    TermSequence::BackgroundColorEnd => "\x1b[49m".to_string(),
                    TermSequence::ResetStyles => "\x1b[0m".to_string(),
                    TermSequence::Username => "#(whoami)".to_string(), // Tmux format string
                    TermSequence::HostnameShort => "#H".to_string(),   // Tmux format string
                    TermSequence::CurrentDirectoryFull => "#(pwd)".to_string(), // Tmux format string
                    TermSequence::CurrentDirectoryTilde => {
                        "#(pwd -P | sed \"s|^$HOME|~|\")".to_string()
                    } // Tmux format string with home abbreviation
                    TermSequence::PrivilegedIndicator => "$".to_string(), // Tmux doesn't have a direct equivalent
                    TermSequence::Newline => "\n".to_string(),
                    TermSequence::Literal(s) => s.clone(),
                }
            }
            TermType::Pwsh => {
                // PowerShell uses ANSI escape sequences directly, similar to Tmux, no \[ and \] wrappers needed.
                // Dynamic information placeholders use PowerShell variables/commands.
                match self {
                    TermSequence::Percent => "%%".to_string(), // PowerShell: %% for single %
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
                        NamedColor::Black => "\x1b[30m".to_string(),
                        NamedColor::Red => "\x1b[31m".to_string(),
                        NamedColor::Green => "\x1b[32m".to_string(),
                        NamedColor::Yellow => "\x1b[33m".to_string(),
                        NamedColor::Blue => "\x1b[34m".to_string(),
                        NamedColor::Magenta => "\x1b[35m".to_string(),
                        NamedColor::Cyan => "\x1b[36m".to_string(),
                        NamedColor::White => "\x1b[37m".to_string(),
                        NamedColor::LightBlack => "\x1b[90m".to_string(),
                        NamedColor::LightRed => "\x1b[91m".to_string(),
                        NamedColor::LightGreen => "\x1b[92m".to_string(),
                        NamedColor::LightYellow => "\x1b[93m".to_string(),
                        NamedColor::LightBlue => "\x1b[94m".to_string(),
                        NamedColor::LightMagenta => "\x1b[95m".to_string(),
                        NamedColor::LightCyan => "\x1b[96m".to_string(),
                        NamedColor::LightWhite => "\x1b[97m".to_string(),
                        NamedColor::Code256(code) => format!("\x1b[38;5;{}m", code),
                    },
                    TermSequence::ForegroundColorEnd => "\x1b[39m".to_string(),
                    TermSequence::BackgroundColor(color) => match color {
                        NamedColor::FullColor((r, g, b)) => format!("\x1b[48;2;{};{};{}m", r, g, b),
                        NamedColor::Black => "\x1b[40m".to_string(),
                        NamedColor::Red => "\x1b[41m".to_string(),
                        NamedColor::Green => "\x1b[42m".to_string(),
                        NamedColor::Yellow => "\x1b[43m".to_string(),
                        NamedColor::Blue => "\x1b[44m".to_string(),
                        NamedColor::Magenta => "\x1b[45m".to_string(),
                        NamedColor::Cyan => "\x1b[46m".to_string(),
                        NamedColor::White => "\x1b[47m".to_string(),
                        NamedColor::LightBlack => "\x1b[100m".to_string(),
                        NamedColor::LightRed => "\x1b[101m".to_string(),
                        NamedColor::LightGreen => "\x1b[102m".to_string(),
                        NamedColor::LightYellow => "\x1b[103m".to_string(),
                        NamedColor::LightBlue => "\x1b[104m".to_string(),
                        NamedColor::LightMagenta => "\x1b[105m".to_string(),
                        NamedColor::LightCyan => "\x1b[106m".to_string(),
                        NamedColor::LightWhite => "\x1b[107m".to_string(),
                        NamedColor::Code256(code) => format!("\x1b[48;5;{}m", code),
                    },
                    TermSequence::BackgroundColorEnd => "\x1b[49m".to_string(),
                    TermSequence::ResetStyles => "\x1b[0m".to_string(),
                    TermSequence::Username => "$env:USERNAME".to_string(),
                    TermSequence::HostnameShort => "$env:COMPUTERNAME".to_string(), // For Windows, or use `hostname` for Linux/macOS PowerShell
                    TermSequence::CurrentDirectoryFull => "$(Get-Location).Path".to_string(),
                    TermSequence::CurrentDirectoryTilde => "$( (Get-Location).Path -replace \"^$([System.Environment]::GetFolderPath('UserProfile'))\", \"~\")".to_string(),
                    TermSequence::PrivilegedIndicator => "#".to_string(), // For simplicity, use # for privileged. Check $IsWindows or $IsLinux for actual logic.
                    TermSequence::Newline => "`n".to_string(), // PowerShell newline
                    TermSequence::Literal(s) => s.clone(),
                }
            }
            TermType::Fish => {
                // Fish shell uses ANSI escape sequences directly.
                // Dynamic information placeholders use command substitutions.
                match self {
                    TermSequence::Percent => "%%".to_string(), // Fish: %% for single %
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
                        NamedColor::Black => "\x1b[30m".to_string(),
                        NamedColor::Red => "\x1b[31m".to_string(),
                        NamedColor::Green => "\x1b[32m".to_string(),
                        NamedColor::Yellow => "\x1b[33m".to_string(),
                        NamedColor::Blue => "\x1b[34m".to_string(),
                        NamedColor::Magenta => "\x1b[35m".to_string(),
                        NamedColor::Cyan => "\x1b[36m".to_string(),
                        NamedColor::White => "\x1b[37m".to_string(),
                        NamedColor::LightBlack => "\x1b[90m".to_string(),
                        NamedColor::LightRed => "\x1b[91m".to_string(),
                        NamedColor::LightGreen => "\x1b[92m".to_string(),
                        NamedColor::LightYellow => "\x1b[93m".to_string(),
                        NamedColor::LightBlue => "\x1b[94m".to_string(),
                        NamedColor::LightMagenta => "\x1b[95m".to_string(),
                        NamedColor::LightCyan => "\x1b[96m".to_string(),
                        NamedColor::LightWhite => "\x1b[97m".to_string(),
                        NamedColor::Code256(code) => format!("\x1b[38;5;{}m", code),
                    },
                    TermSequence::ForegroundColorEnd => "\x1b[39m".to_string(),
                    TermSequence::BackgroundColor(color) => match color {
                        NamedColor::FullColor((r, g, b)) => format!("\x1b[48;2;{};{};{}m", r, g, b),
                        NamedColor::Black => "\x1b[40m".to_string(),
                        NamedColor::Red => "\x1b[41m".to_string(),
                        NamedColor::Green => "\x1b[42m".to_string(),
                        NamedColor::Yellow => "\x1b[43m".to_string(),
                        NamedColor::Blue => "\x1b[44m".to_string(),
                        NamedColor::Magenta => "\x1b[45m".to_string(),
                        NamedColor::Cyan => "\x1b[46m".to_string(),
                        NamedColor::White => "\x1b[47m".to_string(),
                        NamedColor::LightBlack => "\x1b[100m".to_string(),
                        NamedColor::LightRed => "\x1b[101m".to_string(),
                        NamedColor::LightGreen => "\x1b[102m".to_string(),
                        NamedColor::LightYellow => "\x1b[103m".to_string(),
                        NamedColor::LightBlue => "\x1b[104m".to_string(),
                        NamedColor::LightMagenta => "\x1b[105m".to_string(),
                        NamedColor::LightCyan => "\x1b[106m".to_string(),
                        NamedColor::LightWhite => "\x1b[107m".to_string(),
                        NamedColor::Code256(code) => format!("\x1b[48;5;{}m", code),
                    },
                    TermSequence::BackgroundColorEnd => "\x1b[49m".to_string(),
                    TermSequence::ResetStyles => "\x1b[0m".to_string(),
                    TermSequence::Username => "(whoami)".to_string(),
                    TermSequence::HostnameShort => "(hostname -s)".to_string(),
                    TermSequence::CurrentDirectoryFull => "(pwd)".to_string(),
                    TermSequence::CurrentDirectoryTilde => "(prompt_pwd)".to_string(),
                    TermSequence::PrivilegedIndicator => "$".to_string(),
                    TermSequence::Newline => "\n".to_string(),
                    TermSequence::Literal(s) => s.clone(),
                }
            }
        }
    }
}
