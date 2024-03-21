#[derive(Debug)]
struct Arguments {
    target: String,
    replacment: String,
    filename: String,
    output: String,
}

use text_colorizer::*;

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <filename> <output>");
}

use std::env;

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}",
            "Error:".red().bold(),
            args.len()
        );
    }

    Arguments {
        target: args[0].clone(),
        replacment: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn main() {
    print_usage()
}
