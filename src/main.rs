use clap::{Arg, ArgMatches, Command, Parser};

use crate::encryption::aes_gen_key;

mod joman;
mod encryption;

fn cli() -> Command {
    clap::Command::new("joman")
        .about("A journal management system CLI")
        .version("0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("initializes a journal system")
        )
        .subcommand(
            Command::new("add")
                .about("adds a new entry to directory")
                .arg(
                    Arg::new("directory`")
                        .help("Path of the target directory")
                        .required(true)
                        .value_name("DIR")
                )
                .arg(
                    Arg::new("file")
                        .help("Path of the entry file to add")
                        .required(true)
                        .value_name("FILE"),
                ),
                
        )
        .subcommand(
            Command::new("read")
                .about("read an entry in the directory")
                .arg(
                    Arg::new("file")
                        .help("name of the entry file")
                )
        )
        .subcommand(
            Command::new("lock")
                .arg(
                    Arg::new("directory")
                        .help("directory to encrypt")
                        .required(true)
                        .value_name("DIR"),
                )
                .arg(
                    Arg::new("key")
                        .help("AES-256 key")
                        .value_name("KEY"),
                )
                .about("encrypts a directory of entries and returns the key"),
        )
        .subcommand(
            Command::new("generate")
                .about("generates a AES-256 key")
        )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {}
        Some(("add", sub_matches)) => {}
        Some(("read", sub_matches)) => {}
        Some(("lock", sub_matches)) => {}
        Some(("generate", sub_matches)) => {
            let key = aes_gen_key();
            println!("generated key: {}", key)
        }
        _ => !unreachable!(),
    }

    Ok(())
}
