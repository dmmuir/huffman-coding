#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;

mod bytes;
mod cli;
mod compress;
mod format;
mod huffman_tree;
mod stats;

use clap::ArgMatches;

use std::fs;
use std::{
    io,
    io::{Read, Write},
};

use compress::{decode, encode};

const FILE_READ_FAILED: &str = "Problem reading from source";

fn main() {
    let matches = cli::app();

    let mut filename = filename(&matches);
    let command = command(&matches);

    let output = match command {
        Command::Process => {
            let source = source(&matches).expect(FILE_READ_FAILED);
            if matches.is_present("decode") {
                filename = filename.map(|name| name.replace(".huff", ""));
                Some(decode(&source))
            } else {
                filename = filename.map(|name| format!("{}.huff", name));
                Some(encode(&source))
            }
        }
        Command::Stats(file) => {
            let source = fs::read(file).expect(FILE_READ_FAILED);
            stats::print(&source);
            None
        }
    };

    if let Some(output) = output {
        if matches.is_present("stats") {
            stats::print(&output);
        } else if let Some(destination_file) = filename {
            fs::write(destination_file, output).unwrap();
        } else {
            io::stdout().write_all(&output).unwrap();
        }
    }
}

fn source(matches: &ArgMatches) -> io::Result<Vec<u8>> {
    if let Some(input_file) = matches.value_of("filepath") {
        return fs::read(input_file);
    }

    let mut source = Vec::new();
    let mut stdin = io::stdin();
    stdin.read_to_end(&mut source)?;
    Ok(source)
}

fn filename(matches: &ArgMatches) -> Option<String> {
    matches.value_of("filepath").map(|f| f.to_owned())
}

fn command(matches: &ArgMatches) -> Command {
    match matches.subcommand() {
        ("", None) => Command::Process,
        ("stats", Some(file)) => Command::Stats(file.value_of("file").unwrap().to_string()),
        _ => unreachable!(),
    }
}

enum Command {
    Process,
    Stats(String),
}
