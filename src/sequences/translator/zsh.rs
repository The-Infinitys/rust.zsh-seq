use super::super::NamedColor;
use super::super::TermSequence;

pub fn to_zsh_string(sequence: &TermSequence) -> String {
    match sequence {
        TermSequence::Percent => "%%".to_string(), // Zshでリテラルの%は %%
        TermSequence::BoldStart => "%B".to_string(),
        TermSequence::BoldEnd => "%b".to_string(),
        TermSequence::UnderlineStart => "%U".to_string(),
        TermSequence::UnderlineEnd => "%u".to_string(),
        TermSequence::StandoutStart => "%S".to_string(),
        TermSequence::StandoutEnd => "%s".to_string(),
        // SGRエスケープは %{ ... %} で囲む
        TermSequence::StrikethroughStart => "%{\x1b[9m%}".to_string(),
        TermSequence::StrikethroughEnd => "%{\x1b[29m%}".to_string(),
        TermSequence::OverlineStart => "%{\x1b[53m%}".to_string(),
        TermSequence::OverlineEnd => "%{\x1b[55m%}".to_string(),
        TermSequence::BlinkStart => "%{\x1b[5m%}".to_string(),
        TermSequence::BlinkEnd => "%{\x1b[25m%}".to_string(),
        TermSequence::ForegroundColor(color) => match color {
            NamedColor::FullColor((r, g, b)) => {
                format!("%{{\x1b[38;2;{};{};{}m%}}", r, g, b)
            }
            _ => format!("%F{{{}}}", color.to_term_string()),
        },
        TermSequence::ForegroundColorEnd => "%f".to_string(),
        TermSequence::BackgroundColor(color) => match color {
            NamedColor::FullColor((r, g, b)) => {
                format!("%{{\x1b[48;2;{};{};{}m%}}", r, g, b)
            }
            _ => format!("%K{{{}}}", color.to_term_string()),
        },
        TermSequence::BackgroundColorEnd => "%k".to_string(),
        TermSequence::ResetStyles => "%{\x1b[0m%}".to_string(),
        TermSequence::Username => "%n".to_string(),
        TermSequence::HostnameShort => "%m".to_string(),
        TermSequence::CurrentDirectoryFull => "%/".to_string(),
        TermSequence::CurrentDirectoryTilde => "%~".to_string(),
        TermSequence::PrivilegedIndicator => "%#".to_string(),
        TermSequence::Newline => "\n".to_string(),
        TermSequence::Literal(s) => s.replace('%', "%%"), // リテラル内の%をエスケープ
    }
}
