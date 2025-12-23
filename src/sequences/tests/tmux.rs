use crate::builder::TermType;
use crate::colors::NamedColor;
use crate::sequences::TermSequence;

#[test]
fn test_tmux_percent_sequence() {
    assert_eq!(TermSequence::Percent.to_shell_string(TermType::Tmux), "%");
}

#[test]
fn test_tmux_bold_sequences() {
    assert_eq!(
        TermSequence::BoldStart.to_shell_string(TermType::Tmux),
        "\x1b[1m"
    );
    assert_eq!(
        TermSequence::BoldEnd.to_shell_string(TermType::Tmux),
        "\x1b[22m"
    );
}

#[test]
fn test_tmux_underline_sequences() {
    assert_eq!(
        TermSequence::UnderlineStart.to_shell_string(TermType::Tmux),
        "\x1b[4m"
    );
    assert_eq!(
        TermSequence::UnderlineEnd.to_shell_string(TermType::Tmux),
        "\x1b[24m"
    );
}

#[test]
fn test_tmux_foreground_color_sequence() {
    assert_eq!(
        TermSequence::ForegroundColor(NamedColor::Red).to_shell_string(TermType::Tmux),
        "\x1b[31m"
    );
    assert_eq!(
        TermSequence::ForegroundColor(NamedColor::Code256(123)).to_shell_string(TermType::Tmux),
        "\x1b[38;5;123m"
    );
    assert_eq!(
        TermSequence::ForegroundColorEnd.to_shell_string(TermType::Tmux),
        "\x1b[39m"
    );
}

#[test]
fn test_tmux_background_color_sequence() {
    assert_eq!(
        TermSequence::BackgroundColor(NamedColor::Blue).to_shell_string(TermType::Tmux),
        "\x1b[44m"
    );
    assert_eq!(
        TermSequence::BackgroundColor(NamedColor::Code256(200)).to_shell_string(TermType::Tmux),
        "\x1b[48;5;200m"
    );
    assert_eq!(
        TermSequence::BackgroundColorEnd.to_shell_string(TermType::Tmux),
        "\x1b[49m"
    );
}

#[test]
fn test_tmux_reset_styles_sequence() {
    assert_eq!(
        TermSequence::ResetStyles.to_shell_string(TermType::Tmux),
        "\x1b[0m"
    );
}

#[test]
fn test_tmux_username_sequence() {
    assert_eq!(
        TermSequence::Username.to_shell_string(TermType::Tmux),
        r"#(whoami)"
    );
}

#[test]
fn test_tmux_hostname_short_sequence() {
    assert_eq!(
        TermSequence::HostnameShort.to_shell_string(TermType::Tmux),
        r"#H"
    );
}

#[test]
fn test_tmux_current_directory_full_sequence() {
    assert_eq!(
        TermSequence::CurrentDirectoryFull.to_shell_string(TermType::Tmux),
        r"#(pwd)"
    );
}

#[test]
fn test_tmux_current_directory_tilde_sequence() {
    assert_eq!(
        TermSequence::CurrentDirectoryTilde.to_shell_string(TermType::Tmux),
        r#"#(pwd -P | sed "s|^$HOME|~|")"#
    );
}

#[test]
fn test_tmux_privileged_indicator_sequence() {
    assert_eq!(
        TermSequence::PrivilegedIndicator.to_shell_string(TermType::Tmux),
        "$"
    );
}

#[test]
fn test_tmux_literal_sequence() {
    assert_eq!(
        TermSequence::Literal("hello".to_string()).to_shell_string(TermType::Tmux),
        "hello"
    );
}
