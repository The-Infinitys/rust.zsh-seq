use crate::colors::NamedColor;
use crate::sequences::ZshSequence;

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

    pub fn str(mut self, text: &str) -> Self {
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
    pub fn newline(mut self) -> Self {
        self.sequences.push(ZshSequence::Newline);
        self
    }

    pub fn build(&self) -> String {
        self.sequences
            .iter()
            .map(|seq| seq.to_string())
            .collect::<String>()
    }

    /// Extracts all literal text segments from the prompt builder and concatenates them.
    ///
    /// This method collects all `ZshSequence::Literal` contents into a single String,
    /// ignoring all other Zsh escape sequences (style, color, dynamic info).
    pub fn text(&self) -> String {
        self.sequences
            .iter()
            .filter_map(|seq| {
                if let ZshSequence::Literal(s) = seq {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::NamedColor;
    use crate::sequences::ZshSequence;

    #[test]
    fn test_builder_simple() {
        let prompt = ZshPromptBuilder::new()
            .str("Hello, ")
            .username()
            .str("! ")
            .current_dir_tilde()
            .str(" ")
            .privileged_indicator()
            .build();
        assert_eq!(prompt, "Hello, %n! %~ %#");
    }

    #[test]
    fn test_builder_colors_and_styles() {
        let prompt = ZshPromptBuilder::new()
            .bold()
            .color(NamedColor::Green)
            .str("Success: ")
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
            .str("ERROR: ")
            .end_color()
            .bold()
            .str("Something went wrong at ")
            .current_dir_full()
            .end_bold()
            .build();
        assert_eq!(prompt, "%F{red}ERROR: %f%BSomething went wrong at %/%b");
    }

    #[test]
    fn test_builder_true_color() {
        let prompt = ZshPromptBuilder::new()
            .rgb_color(255, 100, 0)
            .str("Hello, True Color!")
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
            .str("Dark Background")
            .reset_styles()
            .build();
        assert_eq!(prompt, "%{\x1b[48;2;50;50;50m%}Dark Background%{\x1b[0m%}");
    }

    #[test]
    fn test_builder_with_full_color() {
        let prompt = ZshPromptBuilder::new()
            .color(NamedColor::FullColor((100, 200, 255)))
            .str("Custom RGB")
            .end_color()
            .build();
        assert_eq!(prompt, "%F{100,200,255}Custom RGB%f");
    }

    #[test]
    fn test_extract_literal_text_only_literal() {
        let builder = ZshPromptBuilder::new().str("hello").str(" world");
        assert_eq!(builder.text(), "hello world");
    }

    #[test]
    fn test_extract_literal_text_with_zsh_sequences() {
        let builder = ZshPromptBuilder::new()
            .bold()
            .color(NamedColor::Red)
            .str("Warning: ")
            .hostname_short()
            .str(" at ")
            .current_dir_full()
            .end_color();
        assert_eq!(builder.text(), "Warning:  at ");
    }

    #[test]
    fn test_extract_literal_text_with_multibyte() {
        let builder = ZshPromptBuilder::new().str("日本語").str("text");
        assert_eq!(builder.text(), "日本語text");
    }

    #[test]
    fn test_extract_literal_text_empty() {
        let builder = ZshPromptBuilder::new()
            .bold()
            .username()
            .privileged_indicator();
        assert_eq!(builder.text(), "");
    }
}
