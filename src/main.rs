use clap::{Arg, ArgAction, Command};

fn main() {
    let _matches = Command::new("echor")
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
}
