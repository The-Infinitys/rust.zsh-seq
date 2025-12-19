use zsh_seq::{ColoredZshPrompt, NamedColor, ZshPromptBuilder};

use std::process::Command;

/// Zshのプロンプトエスケープを解釈した結果を、標準出力にそのまま表示するテスト用関数
fn test_output_in_zsh(prompt: &str) {
    let output = Command::new("zsh")
        .arg("-c")
        // print -P はプロンプトシーケンスを解釈して出力するコマンド
        .arg(format!("print -P '{}'", prompt))
        .output()
        .expect("failed to execute zsh");

    if output.status.success() {
        // Zshが解釈した結果（カラーコード等を含む）をターミナルに表示
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
fn main() {
    // 1. Builderで作成した複雑なプロンプト
    let prompt = ZshPromptBuilder::new()
        .bold()
        .color(NamedColor::Green)
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
    print!("Zsh rendered: ");
    test_output_in_zsh(&prompt);

    // 2. トレイトを使った簡便な装飾
    let warning = "Critical Error".red().bold().on_yellow();
    print!("Trait rendered: ");
    test_output_in_zsh(&warning);
}
