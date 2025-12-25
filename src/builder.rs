use regex::Regex;
use unicode_width::UnicodeWidthStr;

use crate::colors::NamedColor;
use crate::sequences::ZshSequence;
use crate::traits::{ShellPromptBuilder, ZshSpecificBuilder}; // 自前のトレイトをインポート

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

    pub fn add_sequence(&mut self, sequence: ZshSequence) -> &mut Self {
        self.sequences.push(sequence);
        self
    }

    pub fn str(&mut self, text: &str) -> &mut Self {
        self.sequences.push(ZshSequence::Literal(text.to_string()));
        self
    }

    pub fn color(&mut self, color: NamedColor) -> &mut Self {
        self.sequences.push(ZshSequence::ForegroundColor(color));
        self
    }

    pub fn color_bg(&mut self, color: NamedColor) -> &mut Self {
        self.sequences.push(ZshSequence::BackgroundColor(color));
        self
    }

    pub fn reset_styles(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::ResetStyles);
        self
    }

    pub fn bold(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::BoldStart);
        self
    }

    pub fn underline(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::UnderlineStart);
        self
    }

    pub fn standout(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::StandoutStart);
        self
    }

    pub fn end_color(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::ForegroundColorEnd);
        self
    }

    pub fn end_color_bg(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::BackgroundColorEnd);
        self
    }

    pub fn end_bold(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::BoldEnd);
        self
    }

    pub fn end_underline(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::UnderlineEnd);
        self
    }

    pub fn end_standout(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::StandoutEnd);
        self
    }

    pub fn username(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::Username);
        self
    }

    pub fn hostname_short(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::HostnameShort);
        self
    }

    pub fn current_dir_full(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::CurrentDirectoryFull);
        self
    }

    pub fn current_dir_tilde(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::CurrentDirectoryTilde);
        self
    }

    pub fn privileged_indicator(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::PrivilegedIndicator);
        self
    }
    pub fn newline(&mut self) -> &mut Self {
        self.sequences.push(ZshSequence::Newline);
        self
    }
    pub fn connect(&mut self, other: &mut ZshPromptBuilder) -> &mut Self {
        self.sequences.append(&mut other.sequences);
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
        let zsh_str = self.build();
        let output = std::process::Command::new("zsh")
            .arg("-c")
            .arg(format!("print -P \"{}\"", zsh_str))
            .output();

        match output {
            Ok(out) => String::from_utf8_lossy(&out.stdout).trim_end().to_string(),
            Err(_) => self.text(),
        }
    }
    pub fn len(&self) -> usize {
        let raw = self.raw_text();
        let re = Regex::new(r"\x1b\[[0-9;]*[mK]").unwrap();
        let s = re.replace_all(&raw, "");
        UnicodeWidthStr::width(s.as_ref())
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl ShellPromptBuilder for ZshPromptBuilder {
    fn add_str(&mut self, text: &str) -> &mut Self {
        self.str(text)
    }
    fn add_color(&mut self, color: NamedColor) -> &mut Self {
        self.color(color)
    }
    fn add_color_bg(&mut self, color: NamedColor) -> &mut Self {
        self.color_bg(color)
    }
    fn add_reset_styles(&mut self) -> &mut Self {
        self.reset_styles()
    }
    fn add_bold(&mut self) -> &mut Self {
        self.bold()
    }
    fn add_underline(&mut self) -> &mut Self {
        self.underline()
    }
    fn add_standout(&mut self) -> &mut Self {
        self.standout()
    }
    fn add_end_color(&mut self) -> &mut Self {
        self.end_color()
    }
    fn add_end_color_bg(&mut self) -> &mut Self {
        self.end_color_bg()
    }
    fn add_end_bold(&mut self) -> &mut Self {
        self.end_bold()
    }
    fn add_end_underline(&mut self) -> &mut Self {
        self.end_underline()
    }
    fn add_end_standout(&mut self) -> &mut Self {
        self.end_standout()
    }
    fn add_connect(&mut self, other: &mut Self) -> &mut Self {
        // <- other: &mut Self に変更
        self.connect(other)
    }
    fn build(&self) -> String {
        self.build()
    }
    fn text(&self) -> String {
        self.text()
    }
}

impl ZshSpecificBuilder for ZshPromptBuilder {
    fn username(&mut self) -> &mut Self {
        self.username()
    }
    fn hostname_short(&mut self) -> &mut Self {
        self.hostname_short()
    }
    fn current_dir_full(&mut self) -> &mut Self {
        self.current_dir_full()
    }
    fn current_dir_tilde(&mut self) -> &mut Self {
        self.current_dir_tilde()
    }
    fn privileged_indicator(&mut self) -> &mut Self {
        self.privileged_indicator()
    }
    fn newline(&mut self) -> &mut Self {
        self.newline()
    }
    fn raw_text(&self) -> String {
        self.raw_text()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
