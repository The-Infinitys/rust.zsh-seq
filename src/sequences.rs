use crate::builder::TermType;
use crate::colors::NamedColor;
#[cfg(test)]
mod tests;
mod translator;
use translator::{bash, fish, pwsh, tmux, zsh};
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
            TermType::Zsh => zsh::to_zsh_string(self),
            TermType::Bash => bash::to_bash_string(self),
            TermType::Tmux => tmux::to_tmux_string(self),
            TermType::Pwsh => pwsh::to_pwsh_string(self),
            TermType::Fish => fish::to_fish_string(self),
        }
    }
}
