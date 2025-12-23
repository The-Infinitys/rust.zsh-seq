use crate::builder::TermType;
use crate::colors::NamedColor;
use crate::sequences::TermSequence;

#[test]
fn test_pwsh_percent_sequence() {
    assert_eq!(TermSequence::Percent.to_shell_string(TermType::Pwsh), "%%");
}

#[test]
fn test_pwsh_bold_sequences() {
    assert_eq!(
        TermSequence::BoldStart.to_shell_string(TermType::Pwsh),
        "\x1b[1m"
    );
    assert_eq!(
        TermSequence::BoldEnd.to_shell_string(TermType::Pwsh),
        "\x1b[22m"
    );
}

#[test]
fn test_pwsh_underline_sequences() {
    assert_eq!(
        TermSequence::UnderlineStart.to_shell_string(TermType::Pwsh),
        "\x1b[4m"
    );
    assert_eq!(
        TermSequence::UnderlineEnd.to_shell_string(TermType::Pwsh),
        "\x1b[24m"
    );
}

#[test]
fn test_pwsh_foreground_color_sequence() {
    assert_eq!(
        TermSequence::ForegroundColor(NamedColor::Red).to_shell_string(TermType::Pwsh),
        "\x1b[31m"
    );
    assert_eq!(
        TermSequence::ForegroundColor(NamedColor::Code256(123)).to_shell_string(TermType::Pwsh),
        "\x1b[38;5;123m"
    );
    assert_eq!(
        TermSequence::ForegroundColorEnd.to_shell_string(TermType::Pwsh),
        "\x1b[39m"
    );
}

#[test]
fn test_pwsh_background_color_sequence() {
    assert_eq!(
        TermSequence::BackgroundColor(NamedColor::Blue).to_shell_string(TermType::Pwsh),
        "\x1b[44m"
    );
    assert_eq!(
        TermSequence::BackgroundColor(NamedColor::Code256(200)).to_shell_string(TermType::Pwsh),
        "\x1b[48;5;200m"
    );
    assert_eq!(
        TermSequence::BackgroundColorEnd.to_shell_string(TermType::Pwsh),
        "\x1b[49m"
    );
}

#[test]
fn test_pwsh_reset_styles_sequence() {
    assert_eq!(
        TermSequence::ResetStyles.to_shell_string(TermType::Pwsh),
        "\x1b[0m"
    );
}

#[test]
fn test_pwsh_username_sequence() {
    assert_eq!(
        TermSequence::Username.to_shell_string(TermType::Pwsh),
        r"$env:USERNAME"
    );
}

#[test]
fn test_pwsh_hostname_short_sequence() {
    assert_eq!(
        TermSequence::HostnameShort.to_shell_string(TermType::Pwsh),
        r"$env:COMPUTERNAME"
    );
}

#[test]
fn test_pwsh_current_directory_full_sequence() {
    assert_eq!(
        TermSequence::CurrentDirectoryFull.to_shell_string(TermType::Pwsh),
        r"$(Get-Location).Path"
    );
}

#[test]
fn test_pwsh_current_directory_tilde_sequence() {
    assert_eq!(
        TermSequence::CurrentDirectoryTilde.to_shell_string(TermType::Pwsh),
        r#"$( (Get-Location).Path -replace "^$([System.Environment]::GetFolderPath('UserProfile'))", "~")"#
    );
}

#[test]
fn test_pwsh_privileged_indicator_sequence() {
    assert_eq!(
        TermSequence::PrivilegedIndicator.to_shell_string(TermType::Pwsh),
        "#"
    );
}

#[test]
fn test_pwsh_literal_sequence() {
    assert_eq!(
        TermSequence::Literal("hello".to_string()).to_shell_string(TermType::Pwsh),
        "hello"
    );
}
