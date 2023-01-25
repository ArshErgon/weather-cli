use std::io::{stdin, Write};

use owo_colors::OwoColorize;

fn prompt_indicator() {
    print!("{}", " weather❯ ".green().bold());
    std::io::stdout()
        .flush()
        .expect("failed to write to standard output");
}

fn read_user_input() -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("failed to read from standard input");

    input
}

pub fn prompt() -> String {
    prompt_indicator();
    read_user_input()
}
