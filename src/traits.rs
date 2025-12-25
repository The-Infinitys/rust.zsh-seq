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

use crate::colors::NamedColor;

pub trait ShellPromptBuilder: Send + Sync {
    fn add_str(&mut self, text: &str) -> &mut Self;
    fn add_color(&mut self, color: NamedColor) -> &mut Self;
    fn add_color_bg(&mut self, color: NamedColor) -> &mut Self;
    fn add_reset_styles(&mut self) -> &mut Self;
    fn add_bold(&mut self) -> &mut Self;
    fn add_underline(&mut self) -> &mut Self;
    fn add_standout(&mut self) -> &mut Self;
    fn add_end_color(&mut self) -> &mut Self;
    fn add_end_color_bg(&mut self) -> &mut Self;
    fn add_end_bold(&mut self) -> &mut Self;
    fn add_end_underline(&mut self) -> &mut Self;
    fn add_end_standout(&mut self) -> &mut Self;
    fn add_connect(&mut self, other: &mut Self) -> &mut Self;

    fn build(&self) -> String;
    fn text(&self) -> String;
}

// Zsh 固有の機能のためのトレイト
pub trait ZshSpecificBuilder: Send + Sync {
    fn username(&mut self) -> &mut Self;
    fn hostname_short(&mut self) -> &mut Self;
    fn current_dir_full(&mut self) -> &mut Self;
    fn current_dir_tilde(&mut self) -> &mut Self;
    fn privileged_indicator(&mut self) -> &mut Self;
    fn newline(&mut self) -> &mut Self;
    fn raw_text(&self) -> String; // self を消費しない
    fn len(&self) -> usize; // self を消費しない
    fn is_empty(&self) -> bool;
}
