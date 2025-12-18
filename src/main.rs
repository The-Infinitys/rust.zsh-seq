use zsh_seq::{ColoredZshPrompt, NamedColor, ZshPromptBuilder};

fn main() {
    // ZshPromptBuilder の使用例
    let prompt_builder_example = ZshPromptBuilder::new()
        .bold()
        .color(NamedColor::Green)
        .username()
        .text("@")
        .hostname_short()
        .end_bold()
        .text(" ")
        .color(NamedColor::Blue)
        .current_dir_tilde()
        .end_color()
        .text(" ")
        .privileged_indicator()
        .text(" ")
        .build();

    println!("Builder example: {}", prompt_builder_example);

    // ColoredZshPrompt トレイトの使用例
    let trait_example_red = "Error: Something went wrong!".red().bold();
    println!("Trait example (red bold): {}", trait_example_red);

    let trait_example_green_on_yellow = "Success!".green().on_yellow();
    println!("Trait example (green on yellow): {}", trait_example_green_on_yellow);

    let trait_example_rgb = "True Color Text".rgb_color(255, 100, 0).underline();
    println!("Trait example (RGB underline): {}", trait_example_rgb);

    let trait_example_rgb_bg = "RGB Background".on_rgb_color(50, 50, 50).white();
    println!("Trait example (RGB background white text): {}", trait_example_rgb_bg);

    let combined_example = format!(
        "{}{}{}",
        "Current user: ".green(),
        zsh_seq::ZshSequence::Username.to_string().bold(), // ZshSequence直接利用
        " in ".on_blue().white(),
    );
    println!("Combined example: {}", combined_example);
}
