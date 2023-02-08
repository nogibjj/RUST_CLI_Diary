use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::OpenOptions;

pub fn write_diary(entry: &str) {
    let path = Path::new("diary.txt");
    let display = path.display();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .unwrap_or_else(|why| panic!("couldn't open {}: {}", display, why));

    let date_time = chrono::Local::now();
    let now = date_time.format("%Y-%m-%d %H:%M:%S");
    let diary = format!("{} ({})\n", entry, now);

    match file.write_all(diary.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

// read the diary entry based on date and time

pub fn read_diary() {
    let file = File::open("diary.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}


