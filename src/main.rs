use term_seq::{ColoredTermPrompt, NamedColor, TermPromptBuilder};

use std::process::Command;

/// Termのプロンプトエスケープを解釈した結果を、標準出力にそのまま表示するテスト用関数
fn test_output_in_term(prompt: &str) {
    let output = Command::new("term")
        .arg("-c")
        // print -P はプロンプトシーケンスを解釈して出力するコマンド
        .arg(format!("print -P '{}'", prompt))
        .output()
        .expect("failed to execute term");

    if output.status.success() {
        // Termが解釈した結果（カラーコード等を含む）をターミナルに表示
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
fn main() {
    // 1. Builderで作成した複雑なプロンプト
    let prompt = TermPromptBuilder::default()
        .bold()
        .color(NamedColor::FullColor((0, 255, 0)))
        .username()
        .str("@")
        .hostname_short()
        .end_bold()
        .str(" ")
        .current_dir_tilde()
        .str(" ")
        .privileged_indicator()
        .build();

    println!("Raw string: {}", prompt);
    print!("Term rendered: ");
    test_output_in_term(&prompt);

    // 2. トレイトを使った簡便な装飾
    let warning = "Critical Error".red().bold().on_yellow();
    print!("Trait rendered: ");
    test_output_in_term(&warning);
}
