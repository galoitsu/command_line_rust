use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("galoitsu <galoitsu@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .num_args(0),
        )
        .get_matches();

    let texts = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|text| text.to_owned())
        .collect::<Vec<String>>();
    let omit_newline = matches.get_one::<bool>("omit_newline").unwrap();

    print!(
        "{}{}",
        texts.join(" "),
        if *omit_newline { "" } else { "\n" }
    );
}
