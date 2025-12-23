use crate::sequences::TermSequence;
use crate::colors::NamedColor;
use crate::builder::TermType;

#[test]
fn test_fish_percent_sequence() {
    assert_eq!(TermSequence::Percent.to_shell_string(TermType::Fish), "%%");
}

#[test]
fn test_fish_bold_sequences() {
    assert_eq!(TermSequence::BoldStart.to_shell_string(TermType::Fish), "\x1b[1m");
    assert_eq!(TermSequence::BoldEnd.to_shell_string(TermType::Fish), "\x1b[22m");
}

#[test]
fn test_fish_underline_sequences() {
    assert_eq!(
        TermSequence::UnderlineStart.to_shell_string(TermType::Fish),
        "\x1b[4m"
    );
    assert_eq!(
        TermSequence::UnderlineEnd.to_shell_string(TermType::Fish),
        "\x1b[24m"
    );
}

#[test]
fn test_fish_decoration_sequences() {
    assert_eq!(TermSequence::StrikethroughStart.to_shell_string(TermType::Fish), "\x1b[9m");
    assert_eq!(TermSequence::StrikethroughEnd.to_shell_string(TermType::Fish), "\x1b[29m");
    assert_eq!(TermSequence::OverlineStart.to_shell_string(TermType::Fish), "\x1b[53m");
    assert_eq!(TermSequence::OverlineEnd.to_shell_string(TermType::Fish), "\x1b[55m");
    assert_eq!(TermSequence::BlinkStart.to_shell_string(TermType::Fish), "\x1b[5m");
    assert_eq!(TermSequence::BlinkEnd.to_shell_string(TermType::Fish), "\x1b[25m");
}

#[test]
fn test_fish_foreground_color_sequence() {
    assert_eq!(
        TermSequence::ForegroundColor(NamedColor::Red).to_shell_string(TermType::Fish),
        "\x1b[31m"
    );
    assert_eq!(
        TermSequence::ForegroundColor(NamedColor::Code256(123)).to_shell_string(TermType::Fish),
        "\x1b[38;5;123m"
    );
    assert_eq!(
        TermSequence::ForegroundColorEnd.to_shell_string(TermType::Fish),
        "\x1b[39m"
    );
}

#[test]
fn test_fish_background_color_sequence() {
    assert_eq!(
        TermSequence::BackgroundColor(NamedColor::Blue).to_shell_string(TermType::Fish),
        "\x1b[44m"
    );
    assert_eq!(
        TermSequence::BackgroundColor(NamedColor::Code256(200)).to_shell_string(TermType::Fish),
        "\x1b[48;5;200m"
    );
    assert_eq!(
        TermSequence::BackgroundColorEnd.to_shell_string(TermType::Fish),
        "\x1b[49m"
    );
}

#[test]
fn test_fish_reset_styles_sequence() {
    assert_eq!(
        TermSequence::ResetStyles.to_shell_string(TermType::Fish),
        "\x1b[0m"
    );
}

#[test]
fn test_fish_username_sequence() {
    assert_eq!(
        TermSequence::Username.to_shell_string(TermType::Fish),
        "(whoami)"
    );
}

#[test]
fn test_fish_hostname_short_sequence() {
    assert_eq!(
        TermSequence::HostnameShort.to_shell_string(TermType::Fish),
        "(hostname -s)"
    );
}

#[test]
fn test_fish_current_directory_full_sequence() {
    assert_eq!(
        TermSequence::CurrentDirectoryFull.to_shell_string(TermType::Fish),
        "(pwd)"
    );
}

#[test]
fn test_fish_current_directory_tilde_sequence() {
    assert_eq!(
        TermSequence::CurrentDirectoryTilde.to_shell_string(TermType::Fish),
        "(prompt_pwd)"
    );
}

#[test]
fn test_fish_privileged_indicator_sequence() {
    assert_eq!(
        TermSequence::PrivilegedIndicator.to_shell_string(TermType::Fish),
        "$"
    );
}

#[test]
fn test_fish_literal_sequence() {
    assert_eq!(
        TermSequence::Literal("hello".to_string()).to_shell_string(TermType::Fish),
        "hello"
    );
}
