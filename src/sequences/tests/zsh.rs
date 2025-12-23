use crate::builder::TermType;
use crate::colors::NamedColor;
use crate::sequences::TermSequence;

#[test]
fn test_zsh_percent_sequence() {
    assert_eq!(TermSequence::Percent.to_shell_string(TermType::Zsh), "%%%");
}

#[test]
fn test_zsh_bold_sequences() {
    assert_eq!(TermSequence::BoldStart.to_shell_string(TermType::Zsh), "%B");
    assert_eq!(TermSequence::BoldEnd.to_shell_string(TermType::Zsh), "%b");
}

#[test]
fn test_zsh_underline_sequences() {
    assert_eq!(
        TermSequence::UnderlineStart.to_shell_string(TermType::Zsh),
        "%U"
    );
    assert_eq!(
        TermSequence::UnderlineEnd.to_shell_string(TermType::Zsh),
        "%u"
    );
}

#[test]
fn test_zsh_foreground_color_sequence() {
    assert_eq!(
        TermSequence::ForegroundColor(NamedColor::Red).to_shell_string(TermType::Zsh),
        "%F{red}"
    );
    assert_eq!(
        TermSequence::ForegroundColor(NamedColor::Code256(123)).to_shell_string(TermType::Zsh),
        "%F{123}"
    );
    assert_eq!(
        TermSequence::ForegroundColorEnd.to_shell_string(TermType::Zsh),
        "%f"
    );
}

#[test]
fn test_zsh_background_color_sequence() {
    assert_eq!(
        TermSequence::BackgroundColor(NamedColor::Blue).to_shell_string(TermType::Zsh),
        "%K{blue}"
    );
    assert_eq!(
        TermSequence::BackgroundColor(NamedColor::Code256(200)).to_shell_string(TermType::Zsh),
        "%K{200}"
    );
    assert_eq!(
        TermSequence::BackgroundColorEnd.to_shell_string(TermType::Zsh),
        "%k"
    );
}

#[test]
fn test_zsh_reset_styles_sequence() {
    assert_eq!(
        TermSequence::ResetStyles.to_shell_string(TermType::Zsh),
        "%{\x1b[0m%}"
    );
}

#[test]
fn test_zsh_username_sequence() {
    assert_eq!(TermSequence::Username.to_shell_string(TermType::Zsh), "%n");
}

#[test]
fn test_zsh_hostname_short_sequence() {
    assert_eq!(
        TermSequence::HostnameShort.to_shell_string(TermType::Zsh),
        "%m"
    );
}

#[test]
fn test_zsh_current_directory_full_sequence() {
    assert_eq!(
        TermSequence::CurrentDirectoryFull.to_shell_string(TermType::Zsh),
        "%/"
    );
}

#[test]
fn test_zsh_current_directory_tilde_sequence() {
    assert_eq!(
        TermSequence::CurrentDirectoryTilde.to_shell_string(TermType::Zsh),
        "%~"
    );
}

#[test]
fn test_zsh_privileged_indicator_sequence() {
    assert_eq!(
        TermSequence::PrivilegedIndicator.to_shell_string(TermType::Zsh),
        "%#"
    );
}

#[test]
fn test_zsh_literal_sequence() {
    assert_eq!(
        TermSequence::Literal("hello".to_string()).to_shell_string(TermType::Zsh),
        "hello"
    );
}
