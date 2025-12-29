use regex::Regex;
use unicode_width::UnicodeWidthStr;

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
    pub fn connect(mut self, other: Self) -> Self {
        self.sequences.extend(other.sequences);
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
    pub fn raw_text(&self) -> String {
        // let zsh_str = self.build();
        // let output = std::process::Command::new("zsh")
        //     .arg("-c")
        //     .arg(format!("print -P \"{}\"", zsh_str))
        //     .output();

        // match output {
        //     Ok(out) => String::from_utf8_lossy(&out.stdout).trim_end().to_string(),
        //     Err(_) => self.text(),
        // }
        self.sequences
            .iter()
            .map(|seg| {
                let s = seg.raw_text();
                eprintln!("{}", s);
                s
            })
            .collect::<String>()
    }
    pub fn len(&self) -> usize {
        let raw = self.raw_text();
        eprint!("{}", raw);
        let re = Regex::new(r"\x1b\[[0-9;]*[mK]").unwrap();
        let s = re.replace_all(&raw, "");
        UnicodeWidthStr::width(s.as_ref())
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::NamedColor;

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
    fn test_builder_with_full_color() {
        let prompt = ZshPromptBuilder::new()
            .color(NamedColor::FullColor((100, 200, 255)))
            .str("Custom RGB")
            .reset_styles()
            .build();
        assert_eq!(prompt, "%{\x1b[38;2;100;200;255m%}Custom RGB%{\x1b[0m%}");
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
    #[test]
    fn test_builder_connect() {
        let part1 = ZshPromptBuilder::new()
            .color(NamedColor::Blue)
            .str("[")
            .end_color();

        let part2 = ZshPromptBuilder::new().username().str("@").hostname_short();

        let part3 = ZshPromptBuilder::new()
            .color(NamedColor::Blue)
            .str("]")
            .end_color();

        // 3つのビルダーを結合
        let prompt = ZshPromptBuilder::new()
            .connect(part1)
            .connect(part2)
            .connect(part3)
            .build();

        // 期待される出力: %F{blue}[%f%n@%m%F{blue}]%f
        assert_eq!(prompt, "%F{blue}[%f%n@%m%F{blue}]%f");
    }
}
