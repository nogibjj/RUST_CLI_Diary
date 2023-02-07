use diary::{read_diary, write_diary_entry};
use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a diary entry");
        process::exit(1);
    }

    let entry = &args[1..].join(" ");
    write_diary_entry(entry);
    read_diary();
}