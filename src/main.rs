 use clap::{App, Arg, SubCommand};
use diary::{write_diary, read_diary};

fn main() {
    let matches = App::new("Diary")
        .version("0.1.0")
        .author("Emma Wang")
        .about("A simple diary application")
        .subcommand(
            SubCommand::with_name("write")
                .about("write a diary entry")
                .arg(
                    Arg::with_name("entry")
                        .help("the diary entry")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(SubCommand::with_name("read").about("read the diary"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("write") {
        let entry = matches.value_of("entry").unwrap();
        write_diary(entry);
    }

    if let Some(_matches) = matches.subcommand_matches("read") {
        read_diary();
    }
}