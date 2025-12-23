use clap::Parser;
use term_seq::{NamedColor, TermPromptBuilder, TermSequence, TermType};

/// A simple CLI tool to generate terminal escape sequences.
#[derive(Parser, Debug)]
#[command(author, version, about = "Generate terminal escape sequences", long_about = None)]
struct Cli {
    /// Specify the shell type (zsh, bash, tmux, pwsh, fish)
    #[arg(long, short, default_value = "zsh")]
    shell: String,

    /// Add bold formatting
    #[arg(long, short)]
    bold: bool,

    /// Add underline formatting
    #[arg(long, short)]
    underline: bool,

    /// Add strikethrough formatting
    #[arg(long)]
    strikethrough: bool,

    /// Add overline formatting
    #[arg(long)]
    overline: bool,

    /// Add blinking effect
    #[arg(long)]
    blink: bool,

    /// Specify foreground color by name (red, blue, etc.) or 256-code
    #[arg(long, short)]
    fg_color: Option<String>,

    /// Specify background color by name (red, blue, etc.) or 256-code
    #[arg(long)]
    bg_color: Option<String>,

    /// Display username
    #[arg(long)]
    username: bool,

    /// Display short hostname
    #[arg(long)]
    hostname: bool,

    /// Display full current directory
    #[arg(long)]
    cwd_full: bool,

    /// Display current directory with tilde expansion
    #[arg(long)]
    cwd_tilde: bool,

    /// Display privileged indicator (# or $)
    #[arg(long)]
    privileged: bool,

    /// Add a literal string
    text: Vec<String>,

    /// Add a newline character
    #[arg(long, short)]
    newline: bool,

    /// Reset all styles at the end
    #[arg(long, short)]
    reset: bool,
}

fn parse_color(color_str: &str) -> Option<NamedColor> {
    if let Ok(code) = color_str.parse::<u8>() {
        Some(NamedColor::Code256(code))
    } else {
        match color_str.to_lowercase().as_str() {
            "black" => Some(NamedColor::Black),
            "red" => Some(NamedColor::Red),
            "green" => Some(NamedColor::Green),
            "yellow" => Some(NamedColor::Yellow),
            "blue" => Some(NamedColor::Blue),
            "magenta" => Some(NamedColor::Magenta),
            "cyan" => Some(NamedColor::Cyan),
            "white" => Some(NamedColor::White),
            "lightblack" => Some(NamedColor::LightBlack),
            "lightred" => Some(NamedColor::LightRed),
            "lightgreen" => Some(NamedColor::LightGreen),
            "lightyellow" => Some(NamedColor::LightYellow),
            "lightblue" => Some(NamedColor::LightBlue),
            "lightmagenta" => Some(NamedColor::LightMagenta),
            "lightcyan" => Some(NamedColor::LightCyan),
            "lightwhite" => Some(NamedColor::LightWhite),
            _ => None,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let term_type = match cli.shell.to_lowercase().as_str() {
        "zsh" => TermType::Zsh,
        "bash" => TermType::Bash,
        "tmux" => TermType::Tmux,
        "pwsh" => TermType::Pwsh,
        "fish" => TermType::Fish,
        _ => {
            eprintln!("Unknown shell type: {}. Defaulting to zsh.", cli.shell);
            TermType::Zsh
        }
    };

    let mut builder = TermPromptBuilder::new(term_type);

    if cli.bold {
        builder.bold();
    }
    if cli.underline {
        builder.underline();
    }
    if cli.strikethrough {
        builder.add_sequence(TermSequence::StrikethroughStart);
    }
    if cli.overline {
        builder.add_sequence(TermSequence::OverlineStart);
    }
    if cli.blink {
        builder.add_sequence(TermSequence::BlinkStart);
    }
    if let Some(color_str) = cli.fg_color {
        if let Some(color) = parse_color(&color_str) {
            builder.color(color);
        } else {
            eprintln!("Unknown foreground color: {}", color_str);
        }
    }
    if let Some(color_str) = cli.bg_color {
        if let Some(color) = parse_color(&color_str) {
            builder.color_bg(color);
        } else {
            eprintln!("Unknown background color: {}", color_str);
        }
    }

    if cli.username {
        builder.username();
    }
    if cli.hostname {
        builder.hostname_short();
    }
    if cli.cwd_full {
        builder.current_dir_full();
    }
    if cli.cwd_tilde {
        builder.current_dir_tilde();
    }
    if cli.privileged {
        builder.privileged_indicator();
    }

    for text_part in cli.text {
        builder.str(&text_part);
    }

    if cli.newline {
        builder.newline();
    }

    if cli.reset {
        builder.reset_styles();
    }

    let result = builder.build();
    println!("{}", result);

    // The original example's `test_output_in_term` logic is Zsh-specific using `term -c print -P`.
    // For a generic CLI, we just output the generated string.
    // If the user wants to see it rendered, they can pipe it to their shell or use a tool.
    // print!("Term rendered (if piped to a compatible shell): ");
    // test_output_in_term(&result);

    // The second example in the original main() also uses Zsh-specific trait
    // let warning = "Critical Error".red().bold().on_yellow();
    // print!("Trait rendered: ");
    // test_output_in_term(&warning);
}
