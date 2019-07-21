#[macro_use]
extern crate clap;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::exit;
use std::time::Instant;

use clap::{App, AppSettings, Arg, SubCommand};

const NANOS_PER_SEC: u32 = 1_000_000_000;

fn file_validator(path: String) -> Result<(), String> {
    if path == "-" {
        Ok(())
    } else {
        match std::fs::metadata(Path::new(path.as_str())) {
            Ok(ref m) if m.is_file() => Ok(()),
            Ok(ref m) if !m.is_file() => Err(format!("{} is not a file", path)),
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                Err(format!("{}: file not found", path))
            }
            Err(e) => Err(e.description().to_string()),
            _ => Err(format!("{}: unable to read file", path)),
        }
    }
}

fn main() {
    let matches = App::new(crate_name!())
        .about("Reads input from stdin and calculates the rate of lines or values seen over time")
        .author(crate_authors!())
        .version(crate_version!())
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("lines")
                .about("Count each line as it appears")
                .arg(
                    Arg::with_name("file")
                        .index(1)
                        .required(false)
                        .default_value("-")
                        .value_name("FILE")
                        .help("With no FILE or if FILE is -, read from standard input"),
                ),
        )
        .subcommand(
            SubCommand::with_name("numbers")
                .about("Parse the input line as a number and use its value in the rate calculation")
                .arg(
                    Arg::with_name("file")
                        .index(1)
                        .required(false)
                        .default_value("-")
                        .value_name("FILE")
                        .validator(file_validator)
                        .help("With no FILE or if FILE is -, read from standard input"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("lines", Some(matches)) => {
            let path = Path::new(matches.value_of("file").unwrap());
            let mut reader: Box<BufRead> = if path == Path::new("-") {
                Box::new(BufReader::new(io::stdin()))
            } else {
                Box::new(BufReader::new(File::open(path).unwrap()))
            };
            let mut buffer = String::new();
            let mut first_seen: Option<Instant> = None;
            let mut last_seen: Option<Instant> = None;
            let mut lines: u64 = 0;
            loop {
                match reader.read_line(&mut buffer) {
                    Ok(size) => {
                        if size == 0 {
                            if first_seen.is_some() && last_seen.is_some() {
                                let duration = last_seen.unwrap() - first_seen.unwrap();
                                println!(
                                    "{} lines/sec",
                                    lines as f64
                                        / ((duration.as_secs() as f64)
                                            + (duration.subsec_nanos() as f64)
                                                / (NANOS_PER_SEC as f64))
                                );
                            }
                            exit(0);
                        }
                        let ts = Instant::now();
                        first_seen = first_seen.or(Some(ts));
                        last_seen = Some(ts);
                        lines += 1;
                    }
                    Err(_) => {
                        eprintln!("Invalid input, ignoring line.");
                    }
                };
                buffer.clear();
            }
        }
        ("numbers", Some(_matches)) => {
            /*
                        .map(|_| {
                            let now = Instant::now();
                            let x = buffer.trim_end_matches("\n");
                            match x.parse::<f64>() {
                                Ok(number) => {
                                    if state.previous_instant.is_some() {
                                        // we have a previous state, calculate the rate
                                        let diff = number - state.previous;
                                        let duration = now - state.previous_instant.unwrap();
                                        let rate = diff / ((duration.as_secs() as f64) + (duration.subsec_nanos() as f64) / (NANOS_PER_SEC as f64));
                                        println!("{} 1/s", rate)
                                    }
                                    // store the current number
                                    state.previous = number;
                                    state.previous_instant = Some(now);
                                    Ok(number)
                                }
                                Err(e) => {
                                    eprintln!("Ignoring non-numeric input '{}': {}", buffer, e);
                                    Ok(0.0)
                                }
                            }
                        })
            */

            unimplemented!()
        }
        _ => panic!("Missing a required subcommand, this is a bug."),
    }
}
