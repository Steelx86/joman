use clap::{Arg, Command};
use std::error::Error;

mod encryption;
mod joman;

fn cli() -> Command {
    clap::Command::new("joman")
        .about("A journal management system CLI")
        .version("0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("initializes an encrypted journal inside the current directory"),
        )
        .subcommand(
            Command::new("add")
                .about("adds a new entry to directory")
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
                .arg(Arg::new("file").help("name of the entry file")),
        )
        .subcommand(
            Command::new("lock")
                .arg(Arg::new("key").help("AES-256 key").value_name("KEY"))
                .about("encrypts a directory of entries and returns the key"),
        )
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _sub_matches)) => {
            joman::initialize()?;
        }
        Some(("add", sub_matches)) => {
            let file_path = sub_matches
                .get_one::<String>("file")
                .expect("<FILE> argument missing");

            joman::add_file(file_path)?;
        }
        Some(("read", sub_matches)) => {
            let file_path = sub_matches
                .get_one::<String>("file")
                .expect("<FILE> argument missing");

            let content = joman::read_file(file_path, None)?;
            println!("{}", content);
        }
        Some(("lock", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").map(|s| s.as_str());

            joman::lock_directory(key)?;

            println!("Directory locked successfully.");
        }
        _ => unreachable!(),
    }

    Ok(())
}
