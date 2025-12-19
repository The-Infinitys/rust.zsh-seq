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
    /// Start true color foreground (R, G, B) - generates %{\x1b[38;2;R;G;Bm%}
    TrueColorForegroundColor(u8, u8, u8),
    /// Start true color background (R, G, B) - generates %{\x1b[48;2;R;G;Bm%}
    TrueColorBackgroundColor(u8, u8, u8),
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
    /// Custom string that can be inserted directly.
    Literal(String),
}

/// Represents a color for Zsh prompt sequences (named colors or 256-color codes).
#[derive(Debug, Clone, PartialEq, Eq)]
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
}

impl NamedColor {
    /// Converts the NamedColor enum to its string representation for Zsh.
    pub fn to_zsh_string(&self) -> String {
        match self {
            NamedColor::Black => "black".to_string(),
            NamedColor::Red => "red".to_string(),
            NamedColor::Green => "green".to_string(),
            NamedColor::Yellow => "yellow".to_string(),
            NamedColor::Blue => "blue".to_string(),
            NamedColor::Magenta => "magenta".to_string(),
            NamedColor::Cyan => "cyan".to_string(),
            NamedColor::White => "white".to_string(),
            NamedColor::LightBlack => "240".to_string(), // Dark Gray (256-color code)
            NamedColor::LightRed => "lightred".to_string(),
            NamedColor::LightGreen => "lightgreen".to_string(),
            NamedColor::LightYellow => "lightyellow".to_string(),
            NamedColor::LightBlue => "lightblue".to_string(),
            NamedColor::LightMagenta => "lightmagenta".to_string(),
            NamedColor::LightCyan => "lightcyan".to_string(),
            NamedColor::LightWhite => "white".to_string(),
            NamedColor::Code256(code) => code.to_string(),
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
            ZshSequence::ForegroundColor(color) => write!(f, "%F{{{}}}", color.to_zsh_string()),
            ZshSequence::ForegroundColorEnd => write!(f, "%f"),
            ZshSequence::BackgroundColor(color) => write!(f, "%K{{{}}}", color.to_zsh_string()),
            ZshSequence::BackgroundColorEnd => write!(f, "%k"),
            ZshSequence::TrueColorForegroundColor(r, g, b) => {
                write!(f, "%{{\x1b[38;2;{};{};{}m%}}", r, g, b)
            }
            ZshSequence::TrueColorBackgroundColor(r, g, b) => {
                write!(f, "%{{\x1b[48;2;{};{};{}m%}}", r, g, b)
            }
            ZshSequence::ResetStyles => write!(f, "%{{\x1b[0m%}}"),
            ZshSequence::Username => write!(f, "%n"),
            ZshSequence::HostnameShort => write!(f, "%m"),
            ZshSequence::CurrentDirectoryFull => write!(f, "%/"), // Or %d
            ZshSequence::CurrentDirectoryTilde => write!(f, "%~"),
            ZshSequence::PrivilegedIndicator => write!(f, "%#"),
            ZshSequence::Literal(s) => write!(f, "{}", s),
        }
    }
}
pub trait ColoredZshPrompt {
    fn bold(self) -> String;
    fn underline(self) -> String;
    fn red(self) -> String;
    fn green(self) -> String;
    fn yellow(self) -> String;
    fn blue(self) -> String;
    fn white(self) -> String;
    fn on_yellow(self) -> String;
    fn on_blue(self) -> String;
    fn rgb_color(self, r: u8, g: u8, b: u8) -> String;
    fn on_rgb_color(self, r: u8, g: u8, b: u8) -> String;
}

impl<T: AsRef<str>> ColoredZshPrompt for T {
    fn bold(self) -> String {
        format!("%B{}%b", self.as_ref())
    }

    fn underline(self) -> String {
        format!("%U{}%u", self.as_ref())
    }

    fn red(self) -> String {
        format!("%F{{red}}{}%f", self.as_ref())
    }

    fn green(self) -> String {
        format!("%F{{green}}{}%f", self.as_ref())
    }

    fn yellow(self) -> String {
        format!("%F{{yellow}}{}%f", self.as_ref())
    }

    fn blue(self) -> String {
        format!("%F{{blue}}{}%f", self.as_ref())
    }

    fn white(self) -> String {
        format!("%F{{white}}{}%f", self.as_ref())
    }

    fn on_yellow(self) -> String {
        format!("%K{{yellow}}{}%k", self.as_ref())
    }

    fn on_blue(self) -> String {
        format!("%K{{blue}}{}%k", self.as_ref())
    }

    fn rgb_color(self, r: u8, g: u8, b: u8) -> String {
        format!(
            "%{{\x1b[38;2;{};{};{}m%}}{}%{{\x1b[0m%}}",
            r,
            g,
            b,
            self.as_ref()
        )
    }

    fn on_rgb_color(self, r: u8, g: u8, b: u8) -> String {
        format!(
            "%{{\x1b[48;2;{};{};{}m%}}{}%{{\x1b[0m%}}",
            r,
            g,
            b,
            self.as_ref()
        )
    }
}

/// A helper struct to build a prompt string
pub struct ZshPromptBuilder {
    sequences: Vec<ZshSequence>,
}

impl Default for ZshPromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ZshPromptBuilder {
    pub fn new() -> Self {
        Self {
            sequences: Vec::new(),
        }
    }

    pub fn add_sequence(mut self, sequence: ZshSequence) -> Self {
        self.sequences.push(sequence);
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.sequences.push(ZshSequence::Literal(text.to_string()));
        self
    }

    pub fn color(mut self, color: NamedColor) -> Self {
        self.sequences.push(ZshSequence::ForegroundColor(color));
        self
    }

    pub fn color_bg(mut self, color: NamedColor) -> Self {
        self.sequences.push(ZshSequence::BackgroundColor(color));
        self
    }

    pub fn rgb_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.sequences
            .push(ZshSequence::TrueColorForegroundColor(r, g, b));
        self
    }

    pub fn rgb_color_bg(mut self, r: u8, g: u8, b: u8) -> Self {
        self.sequences
            .push(ZshSequence::TrueColorBackgroundColor(r, g, b));
        self
    }

    pub fn reset_styles(mut self) -> Self {
        self.sequences.push(ZshSequence::ResetStyles);
        self
    }

    pub fn bold(mut self) -> Self {
        self.sequences.push(ZshSequence::BoldStart);
        self
    }

    pub fn underline(mut self) -> Self {
        self.sequences.push(ZshSequence::UnderlineStart);
        self
    }

    pub fn standout(mut self) -> Self {
        self.sequences.push(ZshSequence::StandoutStart);
        self
    }

    pub fn end_color(mut self) -> Self {
        self.sequences.push(ZshSequence::ForegroundColorEnd);
        self
    }

    pub fn end_color_bg(mut self) -> Self {
        self.sequences.push(ZshSequence::BackgroundColorEnd);
        self
    }

    pub fn end_bold(mut self) -> Self {
        self.sequences.push(ZshSequence::BoldEnd);
        self
    }

    pub fn end_underline(mut self) -> Self {
        self.sequences.push(ZshSequence::UnderlineEnd);
        self
    }

    pub fn end_standout(mut self) -> Self {
        self.sequences.push(ZshSequence::StandoutEnd);
        self
    }

    pub fn username(mut self) -> Self {
        self.sequences.push(ZshSequence::Username);
        self
    }

    pub fn hostname_short(mut self) -> Self {
        self.sequences.push(ZshSequence::HostnameShort);
        self
    }

    pub fn current_dir_full(mut self) -> Self {
        self.sequences.push(ZshSequence::CurrentDirectoryFull);
        self
    }

    pub fn current_dir_tilde(mut self) -> Self {
        self.sequences.push(ZshSequence::CurrentDirectoryTilde);
        self
    }

    pub fn privileged_indicator(mut self) -> Self {
        self.sequences.push(ZshSequence::PrivilegedIndicator);
        self
    }

    pub fn build(&self) -> String {
        self.sequences
            .iter()
            .map(|seq| seq.to_string())
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_true_color_foreground_sequence() {
        assert_eq!(
            ZshSequence::TrueColorForegroundColor(255, 0, 0).to_string(),
            "%{\x1b[38;2;255;0;0m%}"
        );
    }

    #[test]
    fn test_true_color_background_sequence() {
        assert_eq!(
            ZshSequence::TrueColorBackgroundColor(0, 0, 255).to_string(),
            "%{\x1b[48;2;0;0;255m%}"
        );
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

    #[test]
    fn test_builder_simple() {
        let prompt = ZshPromptBuilder::new()
            .text("Hello, ")
            .username()
            .text("! ")
            .current_dir_tilde()
            .text(" ")
            .privileged_indicator()
            .build();
        assert_eq!(prompt, "Hello, %n! %~ %#");
    }

    #[test]
    fn test_builder_colors_and_styles() {
        let prompt = ZshPromptBuilder::new()
            .bold()
            .color(NamedColor::Green)
            .text("Success: ")
            .end_color()
            .end_bold()
            .current_dir_tilde()
            .build();
        assert_eq!(prompt, "%B%F{green}Success: %f%b%~");
    }

    #[test]
    fn test_builder_chaining() {
        let prompt = ZshPromptBuilder::new()
            .color(NamedColor::Red)
            .text("ERROR: ")
            .end_color()
            .bold()
            .text("Something went wrong at ")
            .current_dir_full()
            .end_bold()
            .build();
        assert_eq!(prompt, "%F{red}ERROR: %f%BSomething went wrong at %/%b");
    }

    #[test]
    fn test_builder_true_color() {
        let prompt = ZshPromptBuilder::new()
            .rgb_color(255, 100, 0)
            .text("Hello, True Color!")
            .reset_styles()
            .build();
        assert_eq!(
            prompt,
            "%{\x1b[38;2;255;100;0m%}Hello, True Color!%{\x1b[0m%}"
        );
    }

    #[test]
    fn test_builder_true_color_bg() {
        let prompt = ZshPromptBuilder::new()
            .rgb_color_bg(50, 50, 50)
            .text("Dark Background")
            .reset_styles()
            .build();
        assert_eq!(prompt, "%{\x1b[48;2;50;50;50m%}Dark Background%{\x1b[0m%}");
    }
}
