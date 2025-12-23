use regex::Regex;
use unicode_width::UnicodeWidthStr;

use crate::colors::NamedColor;
use crate::sequences::TermSequence;

/// A type of the shell
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TermType {
    #[default]
    Zsh,
    Bash,
    Tmux,
    Pwsh,
    Fish,
}

impl TermType {
    pub fn detect() -> Self {
        // 1. Check for Tmux
        if let Ok(term_program) = std::env::var("TERM_PROGRAM")
            && term_program.contains("tmux")
        {
            return TermType::Tmux;
        }

        // 2. Check for PowerShell
        if std::env::var("PSModulePath").is_ok()
            || std::env::var("POWERSHELL_DISTRIBUTION_ID").is_ok()
        {
            return TermType::Pwsh;
        }

        // 3. Check for shell type from SHELL environment variable
        if let Ok(shell) = std::env::var("SHELL") {
            if shell.contains("zsh") {
                return TermType::Zsh;
            } else if shell.contains("bash") {
                return TermType::Bash;
            } else if shell.contains("fish") {
                return TermType::Fish;
            }
        }

        // Fallback to Bash as it's a common default on many systems
        TermType::Bash
    }
}
/// A helper struct to build a prompt string
pub struct TermPromptBuilder {
    term_type: TermType,
    sequences: Vec<TermSequence>,
}

impl Default for TermPromptBuilder {
    fn default() -> Self {
        Self::new(TermType::default())
    }
}

impl TermPromptBuilder {
    pub fn new(term_type: TermType) -> Self {
        Self {
            term_type,
            sequences: Vec::new(),
        }
    }

    pub fn add_sequence(&mut self, sequence: TermSequence) -> &mut Self {
        self.sequences.push(sequence);
        self
    }

    pub fn str(&mut self, text: &str) -> &mut Self {
        self.sequences.push(TermSequence::Literal(text.to_string()));
        self
    }

    pub fn color(&mut self, color: NamedColor) -> &mut Self {
        self.sequences.push(TermSequence::ForegroundColor(color));
        self
    }

    pub fn color_bg(&mut self, color: NamedColor) -> &mut Self {
        self.sequences.push(TermSequence::BackgroundColor(color));
        self
    }

    pub fn reset_styles(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::ResetStyles);
        self
    }

    pub fn bold(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::BoldStart);
        self
    }

    pub fn underline(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::UnderlineStart);
        self
    }

    pub fn standout(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::StandoutStart);
        self
    }

    pub fn end_color(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::ForegroundColorEnd);
        self
    }

    pub fn end_color_bg(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::BackgroundColorEnd);
        self
    }

    pub fn end_bold(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::BoldEnd);
        self
    }

    pub fn end_underline(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::UnderlineEnd);
        self
    }

    pub fn end_standout(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::StandoutEnd);
        self
    }

    pub fn username(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::Username);
        self
    }

    pub fn hostname_short(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::HostnameShort);
        self
    }

    pub fn current_dir_full(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::CurrentDirectoryFull);
        self
    }

    pub fn current_dir_tilde(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::CurrentDirectoryTilde);
        self
    }

    pub fn privileged_indicator(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::PrivilegedIndicator);
        self
    }
    pub fn newline(&mut self) -> &mut Self {
        self.sequences.push(TermSequence::Newline);
        self
    }
    pub fn connect(&mut self, other: &mut TermPromptBuilder) -> &mut Self {
        self.sequences.append(&mut other.sequences);
        self
    }
    pub fn build(&self) -> String {
        self.sequences
            .iter()
            .map(|seq| seq.to_shell_string(self.term_type))
            .collect::<String>()
    }

    /// Extracts all literal text segments from the prompt builder and concatenates them.
    ///
    /// This method collects all `TermSequence::Literal` contents into a single String,
    /// ignoring all other Term escape sequences (style, color, dynamic info).
    pub fn text(&self) -> String {
        self.sequences
            .iter()
            .filter_map(|seq| {
                if let TermSequence::Literal(s) = seq {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect::<String>()
    }
    pub fn raw_text(&self) -> String {
        let term_str = self.build();
        match self.term_type {
            TermType::Zsh => {
                let output = std::process::Command::new("term")
                    .arg("-c")
                    .arg(format!("print -P \"{}\"", term_str))
                    .output();

                match output {
                    Ok(out) => String::from_utf8_lossy(&out.stdout).trim_end().to_string(),
                    Err(_) => self.text(), // Fallback to raw text if 'term' command fails
                }
            }
            _ => {
                // For Bash, Tmux, Pwsh, we cannot rely on 'term' command.
                // We need to strip ANSI escape codes.
                let re = Regex::new(r"\x1b\[[0-9;]*[mK]").unwrap();
                re.replace_all(&term_str, "").to_string()
            }
        }
    }
    pub fn len(&self) -> usize {
        let raw = self.raw_text();
        UnicodeWidthStr::width(raw.as_str())
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn set_env(key: &str, value: &str) -> Option<String> {
        let old_value = env::var(key).ok();
        unsafe {
            env::set_var(key, value);
        }
        old_value
    }

    fn restore_env(key: &str, old_value: Option<String>) {
        unsafe {
            if let Some(value) = old_value {
                env::set_var(key, value);
            } else {
                env::remove_var(key);
            }
        }
    }

    #[test]
    fn test_detect_zsh() {
        let old_shell = set_env("SHELL", "/bin/zsh");
        let old_term_program = set_env("TERM_PROGRAM", "iTerm.app"); // Simulate a common setup
        let old_psmodulepath = env::var("PSModulePath").ok();
        unsafe {
            env::remove_var("PSModulePath");
        }
        let old_powershell_distribution_id = env::var("POWERSHELL_DISTRIBUTION_ID").ok();
        unsafe {
            env::remove_var("POWERSHELL_DISTRIBUTION_ID");
        }

        assert_eq!(TermType::detect(), TermType::Zsh);

        restore_env("SHELL", old_shell);
        restore_env("TERM_PROGRAM", old_term_program);
        restore_env("PSModulePath", old_psmodulepath);
        restore_env("POWERSHELL_DISTRIBUTION_ID", old_powershell_distribution_id);
    }

    #[test]
    fn test_detect_bash() {
        let old_shell = set_env("SHELL", "/bin/bash");
        let old_term_program = set_env("TERM_PROGRAM", "");
        let old_psmodulepath = env::var("PSModulePath").ok();
        unsafe {
            env::remove_var("PSModulePath");
        }
        let old_powershell_distribution_id = env::var("POWERSHELL_DISTRIBUTION_ID").ok();
        unsafe {
            env::remove_var("POWERSHELL_DISTRIBUTION_ID");
        }

        assert_eq!(TermType::detect(), TermType::Bash);

        restore_env("SHELL", old_shell);
        restore_env("TERM_PROGRAM", old_term_program);
        restore_env("PSModulePath", old_psmodulepath);
        restore_env("POWERSHELL_DISTRIBUTION_ID", old_powershell_distribution_id);
    }

    #[test]
    fn test_detect_fish() {
        let old_shell = set_env("SHELL", "/usr/bin/fish");
        let old_term_program = set_env("TERM_PROGRAM", "");
        let old_psmodulepath = env::var("PSModulePath").ok();
        unsafe {
            env::remove_var("PSModulePath");
        }
        let old_powershell_distribution_id = env::var("POWERSHELL_DISTRIBUTION_ID").ok();
        unsafe {
            env::remove_var("POWERSHELL_DISTRIBUTION_ID");
        }

        assert_eq!(TermType::detect(), TermType::Fish);

        restore_env("SHELL", old_shell);
        restore_env("TERM_PROGRAM", old_term_program);
        restore_env("PSModulePath", old_psmodulepath);
        restore_env("POWERSHELL_DISTRIBUTION_ID", old_powershell_distribution_id);
    }

    #[test]
    fn test_detect_tmux() {
        let old_shell = set_env("SHELL", "/bin/bash"); // Tmux often runs under bash
        let old_term_program = set_env("TERM_PROGRAM", "tmux");
        let old_psmodulepath = env::var("PSModulePath").ok();
        unsafe {
            env::remove_var("PSModulePath");
        }
        let old_powershell_distribution_id = env::var("POWERSHELL_DISTRIBUTION_ID").ok();
        unsafe {
            env::remove_var("POWERSHELL_DISTRIBUTION_ID");
        }

        assert_eq!(TermType::detect(), TermType::Tmux);

        restore_env("SHELL", old_shell);
        restore_env("TERM_PROGRAM", old_term_program);
        restore_env("PSModulePath", old_psmodulepath);
        restore_env("POWERSHELL_DISTRIBUTION_ID", old_powershell_distribution_id);
    }

    #[test]
    fn test_detect_pwsh() {
        let old_shell = set_env("SHELL", "/bin/bash"); // Pwsh can run under bash
        let old_term_program = set_env("TERM_PROGRAM", "");
        let old_psmodulepath = set_env("PSModulePath", "/path/to/modules");
        let old_powershell_distribution_id = set_env("POWERSHELL_DISTRIBUTION_ID", "Ubuntu");

        assert_eq!(TermType::detect(), TermType::Pwsh);

        restore_env("SHELL", old_shell);
        restore_env("TERM_PROGRAM", old_term_program);
        restore_env("PSModulePath", old_psmodulepath);
        restore_env("POWERSHELL_DISTRIBUTION_ID", old_powershell_distribution_id);
    }

    #[test]
    fn test_detect_default_bash() {
        // Clear all relevant env vars to test default fallback
        let old_shell = env::var("SHELL").ok();
        unsafe {
            env::remove_var("SHELL");
        }
        let old_term_program = env::var("TERM_PROGRAM").ok();
        unsafe {
            env::remove_var("TERM_PROGRAM");
        }
        let old_psmodulepath = env::var("PSModulePath").ok();
        unsafe {
            env::remove_var("PSModulePath");
        }
        let old_powershell_distribution_id = env::var("POWERSHELL_DISTRIBUTION_ID").ok();
        unsafe {
            env::remove_var("POWERSHELL_DISTRIBUTION_ID");
        }

        assert_eq!(TermType::detect(), TermType::Bash);

        restore_env("SHELL", old_shell);
        restore_env("TERM_PROGRAM", old_term_program);
        restore_env("PSModulePath", old_psmodulepath);
        restore_env("POWERSHELL_DISTRIBUTION_ID", old_powershell_distribution_id);
    }
}
