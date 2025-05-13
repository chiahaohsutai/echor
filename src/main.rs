use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Chia Hao Hsu Tai <chiahaohsutai@gmail.com>")
        .about("Echo command written in Rust")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .required(true)
                .num_args(1..)
                .help("Input text"),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print new line"),
        )
        .get_matches();

    let text: Vec<&str> = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|t| t.as_str())
        .collect();

    let omit_newline = matches.get_flag("omit_newline");
    let ending = if omit_newline { "" } else { "\n" };

    println!("{}{}", text.join(" "), ending)
}
