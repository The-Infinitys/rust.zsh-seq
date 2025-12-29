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
    pub fn bash_build(&self)->String{
        self.sequences
            .iter()
            .map(|seq| seq.bash())
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
        self.sequences
            .iter()
            .map(|seq| seq.raw())
            .collect::<Vec<String>>()
            .join("")
    }
    pub fn len(&self) -> usize {
        let raw = self.raw_text();
        let re = Regex::new(r"[\u001b\u009b]\[[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]|[\u001b\u009b][()][A-Z0-9]").unwrap();
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

#[cfg(test)]
mod len_tests {
    use super::*;
    use crate::colors::NamedColor;

    #[test]
    fn test_len_simple_text() {
        let builder = ZshPromptBuilder::new().str("hello");
        assert_eq!(builder.len(), 5);
    }

    #[test]
    fn test_len_with_colors_and_styles() {
        // 色や太字が含まれていても、可視文字数のみをカウントすべき
        let builder = ZshPromptBuilder::new()
            .bold()
            .color(NamedColor::Red)
            .str("Alert")
            .reset_styles();
        assert_eq!(builder.len(), 5);
    }

    #[test]
    fn test_len_with_full_color_rgb() {
        let builder = ZshPromptBuilder::new()
            .color(NamedColor::FullColor((255, 0, 0)))
            .str("RGB")
            .reset_styles();
        // %{\x1b[38;2;...m%} は 0 幅として計算されるべき
        assert_eq!(builder.len(), 3);
    }

    #[test]
    fn test_len_with_multibyte() {
        // 日本語（全角）は unicode-width により 2 幅として計算される
        let builder = ZshPromptBuilder::new().str("こんにちは");
        assert_eq!(builder.len(), 10); // 2 * 5 = 10
    }

    #[test]
    fn test_len_with_mixed_content() {
        let builder = ZshPromptBuilder::new()
            .color(NamedColor::Blue)
            .str("Dir: ")
            .reset_styles()
            .str("🚀"); // 絵文字
        // "Dir: " (5) + "🚀" (2) = 7
        assert_eq!(builder.len(), 7);
    }

    #[test]
    fn test_len_dynamic_content() {
        // Username や CurrentDir は実行環境に依存するため、
        // raw() の結果と直接比較して整合性を確認する
        let builder = ZshPromptBuilder::new().username();
        let expected_raw = builder.raw_text();
        assert_eq!(builder.len(), UnicodeWidthStr::width(expected_raw.as_str()));
    }

    #[test]
    fn test_len_with_newline() {
        // 改行が含まれる場合、表示幅の計算からは除外するのが一般的（プロンプトの長さ計算）
        let builder = ZshPromptBuilder::new().str("Line1").newline().str("Line2");
        assert_eq!(builder.len(), 10);
    }
}

#[test]
fn test_actual_zsh_expansion() {
    let builder = ZshPromptBuilder::new().username();
    let zsh_prompt = builder.build(); // 例: "%n"

    // 実際に zsh を起動してプロンプトを展開させる
    let output = std::process::Command::new("zsh")
        .arg("-c")
        .arg(format!("print -P '{}'", zsh_prompt)) // -P はプロンプト展開フラグ
        .output()
        .expect("Failed to execute zsh");

    let expanded_zsh = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    // Rust側の raw() 実装と、本物の zsh の展開結果を比較
    assert_eq!(builder.raw_text(), expanded_zsh);
}

#[test]
fn test_actual_bash_expansion() {
    // bashの場合、PS1を展開させるのは少し工夫が必要
    let builder = ZshPromptBuilder::new().username();
    let bash_prompt = builder.bash_build(); // \u などを含む文字列

    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("echo -e \"${{PS1@P}}\"")) // ${PS1@P} はプロンプト展開
        .env("PS1", bash_prompt)
        .output()
        .expect("Failed to execute bash");

    let expanded_bash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    assert_eq!(builder.raw_text(), expanded_bash);
}
