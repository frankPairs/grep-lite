use ::clap::{App, Arg}; // Brings clap::app and clap::Arg objects into local scope
use ::regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // Incrementally builds a command argument parser where each argument takes an Arg. In our
    // case, we only need one.
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search.")
                .takes_value(true)
                .required(false),
        )
        .get_matches();
    let pattern = args.value_of("pattern").unwrap(); // Extracts the pattern argument
    let input = args.value_of("input").unwrap_or("-");
    let re = Regex::new(pattern).unwrap();

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();

        process_lines(reader, re);
    } else {
        let file = File::open(input).unwrap();
        let reader = BufReader::new(file);

        process_lines(reader, re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_result in reader.lines() {
        let line = line_result.unwrap();

        // line is a String, but re.find() takes an &str as an argument
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
