use std::{
    error::Error,
    io::{BufRead, BufReader},
};

use clap::{Arg, Command};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type Result<T> = core::result::Result<T, Box<dyn Error>>;

pub fn get_args() -> Result<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("galoitsu <galoitsu@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number lines")
                .num_args(0)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number nonblank lines")
                .num_args(0),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .map(|text| text.to_owned())
            .collect::<Vec<String>>(),
        number_lines: *matches.get_one::<bool>("number_lines").unwrap(),
        number_nonblank_lines: *matches.get_one::<bool>("number_nonblank_lines").unwrap(),
    })
}

pub fn run(config: Config) -> Result<()> {
    for file_name in config.files {
        match open(&file_name) {
            Err(err) => eprintln!("Failed to open {}: {}", file_name, err),
            Ok(file) => {
                let mut line_number = 0;
                for line_result in file.lines() {
                    let line = line_result?;
                    if config.number_lines {
                        line_number += 1;
                        println!("{:>6}\t{}", line_number, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            line_number += 1;
                            println!("{:>6}\t{}", line_number, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }

    Ok(())
}

fn open(file_name: &str) -> Result<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(std::fs::File::open(file_name)?))),
    }
}
