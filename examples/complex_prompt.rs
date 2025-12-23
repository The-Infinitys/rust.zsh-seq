use term_seq::{
    builder::TermType,
    colors::NamedColor,
    TermPromptBuilder,
    TermSequence,
};

fn main() {
    println!("--- Complex Prompt Example ---");

    // Detect current shell type
    let detected_term_type = TermType::detect();
    println!("Detected TermType: {:?}", detected_term_type);

    // Create a builder for the detected shell type
    let mut builder = TermPromptBuilder::new(detected_term_type);

    // Build a complex prompt
    builder
        .color(NamedColor::Green)
        .bold()
        .username()
        .str("@")
        .hostname_short()
        .end_bold()
        .str(":")
        .color(NamedColor::Blue)
        .current_dir_tilde()
        .reset_styles()
        .str(" ")
        .add_sequence(TermSequence::StrikethroughStart)
        .color(NamedColor::Red)
        .str("DEPRECATED")
        .add_sequence(TermSequence::StrikethroughEnd)
        .str(" ")
        .add_sequence(TermSequence::OverlineStart)
        .color(NamedColor::Yellow)
        .str("IMPORTANT")
        .add_sequence(TermSequence::OverlineEnd)
        .str(" ")
        .add_sequence(TermSequence::BlinkStart)
        .color(NamedColor::Magenta)
        .str("ALERT")
        .add_sequence(TermSequence::BlinkEnd)
        .str(" ")
        .privileged_indicator()
        .str(" ");

    let prompt = builder.build();

    println!("\nGenerated Prompt for {:?}:", detected_term_type);
    println!("{}", prompt);

    println!("\n--- Raw Text Example ---");
    // Show how to get raw text (without escape codes)
    let mut raw_builder = TermPromptBuilder::new(detected_term_type);
    raw_builder
        .str("Hello ")
        .bold()
        .str("World")
        .end_bold()
        .str("!");
    let raw_text = raw_builder.text();
    println!("Raw text content: '{}'", raw_text);

    println!("\n--- Minimal Prompt Example ---");
    let mut minimal_builder = TermPromptBuilder::new(detected_term_type);
    minimal_builder
        .color(NamedColor::Cyan)
        .str("$")
        .reset_styles()
        .str(" ");
    let minimal_prompt = minimal_builder.build();
    println!("{}", minimal_prompt);


    println!("\n--- Color Code Example (256-color) ---");
    let mut color_code_builder = TermPromptBuilder::new(detected_term_type);
    color_code_builder
        .color(NamedColor::Code256(160)) // A shade of red
        .str("Code 256 Color")
        .reset_styles();
    let color_code_prompt = color_code_builder.build();
    println!("{}", color_code_prompt);

    println!("\n--- Full Color Example (RGB) ---");
    let mut full_color_builder = TermPromptBuilder::new(detected_term_type);
    full_color_builder
        .color(NamedColor::FullColor((50, 150, 250))) // A shade of blue
        .str("Full RGB Color")
        .reset_styles();
    let full_color_prompt = full_color_builder.build();
    println!("{}", full_color_prompt);
}
