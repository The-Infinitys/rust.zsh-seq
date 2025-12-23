use super::super::NamedColor;
use super::super::TermSequence;

pub fn to_bash_string(sequence: &TermSequence) -> String {
    match sequence {
        TermSequence::Percent => "%%".to_string(),
        // Bashは \[\e[ ... m\] を使用。特定の属性解除コードを使用し、他のスタイルを壊さないようにする
        TermSequence::BoldStart => r"\[\e[1m\]".to_string(),
        TermSequence::BoldEnd => r"\[\e[22m\]".to_string(),
        TermSequence::UnderlineStart => r"\[\e[4m\]".to_string(),
        TermSequence::UnderlineEnd => r"\[\e[24m\]".to_string(),
        TermSequence::StandoutStart => r"\[\e[7m\]".to_string(),
        TermSequence::StandoutEnd => r"\[\e[27m\]".to_string(),
        TermSequence::StrikethroughStart => r"\[\e[9m\]".to_string(),
        TermSequence::StrikethroughEnd => r"\[\e[29m\]".to_string(),
        TermSequence::OverlineStart => r"\[\e[53m\]".to_string(),
        TermSequence::OverlineEnd => r"\[\e[55m\]".to_string(),
        TermSequence::BlinkStart => r"\[\e[5m\]".to_string(),
        TermSequence::BlinkEnd => r"\[\e[25m\]".to_string(),
        TermSequence::ForegroundColor(color) => match color {
            NamedColor::FullColor((r, g, b)) => format!(r"\[\e[38;2;{};{};{}m\]", r, g, b),
            _ => format!(r"\[\e[38;5;{}m\]", color.to_term_string()),
        },
        TermSequence::ForegroundColorEnd => r"\[\e[39m\]".to_string(),
        TermSequence::BackgroundColor(color) => match color {
            NamedColor::FullColor((r, g, b)) => format!(r"\[\e[48;2;{};{};{}m\]", r, g, b),
            _ => format!(r"\[\e[48;5;{}m\]", color.to_term_string()),
        },
        TermSequence::BackgroundColorEnd => r"\[\e[49m\]".to_string(),
        TermSequence::ResetStyles => r"\[\e[0m\]".to_string(),
        TermSequence::Username => r"\u".to_string(),
        TermSequence::HostnameShort => r"\h".to_string(),
        TermSequence::CurrentDirectoryFull => r"\w".to_string(),
        TermSequence::CurrentDirectoryTilde => r"\w".to_string(),
        TermSequence::PrivilegedIndicator => r"\$".to_string(),
        TermSequence::Newline => r"\n".to_string(),
        TermSequence::Literal(s) => s.clone(),
    }
}
