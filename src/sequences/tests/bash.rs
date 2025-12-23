use crate::builder::TermType;
use crate::colors::NamedColor;
use crate::sequences::TermSequence;

#[test]
fn test_bash_percent_sequence() {
    assert_eq!(TermSequence::Percent.to_shell_string(TermType::Bash), "%%");
}

#[test]
fn test_bash_bold_sequences() {
    assert_eq!(
        TermSequence::BoldStart.to_shell_string(TermType::Bash),
        "\\[\\e[1m\\]"
    );
    assert_eq!(
        TermSequence::BoldEnd.to_shell_string(TermType::Bash),
        "\\[\\e[22m\\]"
    );
}

#[test]
fn test_bash_underline_sequences() {
    assert_eq!(
        TermSequence::UnderlineStart.to_shell_string(TermType::Bash),
        "\\[\\e[4m\\]"
    );
    assert_eq!(
        TermSequence::UnderlineEnd.to_shell_string(TermType::Bash),
        "\\[\\e[24m\\]"
    );
}

#[test]
fn test_bash_foreground_color_sequence() {
    assert_eq!(
        TermSequence::ForegroundColor(NamedColor::Red).to_shell_string(TermType::Bash),
        "\\[\\e[31m\\]"
    );
    assert_eq!(
        TermSequence::ForegroundColor(NamedColor::Code256(123)).to_shell_string(TermType::Bash),
        "\\[\\e[38;5;123m\\]"
    );
    assert_eq!(
        TermSequence::ForegroundColorEnd.to_shell_string(TermType::Bash),
        "\\[\\e[39m\\]"
    );
}

#[test]
fn test_bash_background_color_sequence() {
    assert_eq!(
        TermSequence::BackgroundColor(NamedColor::Blue).to_shell_string(TermType::Bash),
        "\\[\\e[44m\\]"
    );
    assert_eq!(
        TermSequence::BackgroundColor(NamedColor::Code256(200)).to_shell_string(TermType::Bash),
        "\\[\\e[48;5;200m\\]"
    );
    assert_eq!(
        TermSequence::BackgroundColorEnd.to_shell_string(TermType::Bash),
        "\\[\\e[49m\\]"
    );
}

#[test]
fn test_bash_reset_styles_sequence() {
    assert_eq!(
        TermSequence::ResetStyles.to_shell_string(TermType::Bash),
        "\\[\\e[0m\\]"
    );
}

#[test]
fn test_bash_username_sequence() {
    assert_eq!(
        TermSequence::Username.to_shell_string(TermType::Bash),
        "\\u"
    );
}

#[test]
fn test_bash_hostname_short_sequence() {
    assert_eq!(
        TermSequence::HostnameShort.to_shell_string(TermType::Bash),
        "\\h"
    );
}

#[test]
fn test_bash_current_directory_full_sequence() {
    assert_eq!(
        TermSequence::CurrentDirectoryFull.to_shell_string(TermType::Bash),
        "\\w"
    );
}

#[test]
fn test_bash_current_directory_tilde_sequence() {
    assert_eq!(
        TermSequence::CurrentDirectoryTilde.to_shell_string(TermType::Bash),
        "\\w"
    );
}

#[test]
fn test_bash_privileged_indicator_sequence() {
    assert_eq!(
        TermSequence::PrivilegedIndicator.to_shell_string(TermType::Bash),
        "\\$"
    );
}

#[test]
fn test_bash_literal_sequence() {
    assert_eq!(
        TermSequence::Literal("hello".to_string()).to_shell_string(TermType::Bash),
        "hello"
    );
}

#[test]
fn test_bash_decoration_sequences() {
    assert_eq!(TermSequence::StrikethroughStart.to_shell_string(TermType::Bash), "\\[\\e[9m\\]");
    assert_eq!(TermSequence::StrikethroughEnd.to_shell_string(TermType::Bash), "\\[\\e[29m\\]");
    assert_eq!(TermSequence::OverlineStart.to_shell_string(TermType::Bash), "\\[\\e[53m\\]");
    assert_eq!(TermSequence::OverlineEnd.to_shell_string(TermType::Bash), "\\[\\e[55m\\]");
    assert_eq!(TermSequence::BlinkStart.to_shell_string(TermType::Bash), "\\[\\e[5m\\]");
    assert_eq!(TermSequence::BlinkEnd.to_shell_string(TermType::Bash), "\\[\\e[25m\\]");
}
