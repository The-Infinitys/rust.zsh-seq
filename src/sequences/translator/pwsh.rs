use super::super::NamedColor;
use super::super::TermSequence;

pub fn to_pwsh_string(sequence: &TermSequence) -> String {
    match sequence {
        TermSequence::Percent => "%".to_string(),
        TermSequence::BoldStart => "\x1b[1m".to_string(),
        TermSequence::BoldEnd => "\x1b[22m".to_string(),
        TermSequence::UnderlineStart => "\x1b[4m".to_string(),
        TermSequence::UnderlineEnd => "\x1b[24m".to_string(),
        TermSequence::StandoutStart => "\x1b[7m".to_string(),
        TermSequence::StandoutEnd => "\x1b[27m".to_string(),
        TermSequence::StrikethroughStart => "\x1b[9m".to_string(),
        TermSequence::StrikethroughEnd => "\x1b[29m".to_string(),
        TermSequence::OverlineStart => "\x1b[53m".to_string(),
        TermSequence::OverlineEnd => "\x1b[55m".to_string(),
        TermSequence::BlinkStart => "\x1b[5m".to_string(),
        TermSequence::BlinkEnd => "\x1b[25m".to_string(),
        TermSequence::ForegroundColor(color) => match color {
            NamedColor::FullColor((r, g, b)) => format!("\x1b[38;2;{};{};{}m", r, g, b),
            _ => format!("\x1b[38;5;{}m", color.to_term_string()),
        },
        TermSequence::ForegroundColorEnd => "\x1b[39m".to_string(),
        TermSequence::BackgroundColor(color) => match color {
            NamedColor::FullColor((r, g, b)) => format!("\x1b[48;2;{};{};{}m", r, g, b),
            _ => format!("\x1b[48;5;{}m", color.to_term_string()),
        },
        TermSequence::BackgroundColorEnd => "\x1b[49m".to_string(),
        TermSequence::ResetStyles => "\x1b[0m".to_string(),

        // シェルごとの動的プレースホルダ
        TermSequence::Username => "$env:USERNAME".to_string(),
        TermSequence::HostnameShort => "$env:COMPUTERNAME".to_string(),
        TermSequence::CurrentDirectoryFull => "$PWD".to_string(),
        TermSequence::CurrentDirectoryTilde => "$(Split-Path $PWD -Leaf)".to_string(), // 簡易例
        TermSequence::PrivilegedIndicator => "$".to_string(),
        TermSequence::Newline => "\n".to_string(),
        TermSequence::Literal(s) => s.clone(),
    }
}
