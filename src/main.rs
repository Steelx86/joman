use clap::{Arg, ArgMatches, Command, Parser};

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
                .about("initializes an encrypted journal inside the current directory")
                .arg(
                    Arg::new("password")
                        .help("password for the journal")
                        .value_name("PASSWORD"),
                ),
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
                .arg(
                    Arg::new("directory")
                        .help("directory to encrypt")
                        .required(true)
                        .value_name("DIR"),
                )
                .arg(Arg::new("key").help("AES-256 key").value_name("KEY"))
                .about("encrypts a directory of entries and returns the key"),
        )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            joman::initialize();
        }
        Some(("add", sub_matches)) => {
            let file_path = sub_matches
                .get_one::<String>("file")
                .expect("file argument missing");

            joman::add_file(file_path);
        }
        Some(("read", sub_matches)) => {

        }
        Some(("lock", sub_matches)) => {

        }
        _ => unreachable!(),
    }

    Ok(())
}
