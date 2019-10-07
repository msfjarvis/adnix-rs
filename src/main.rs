extern crate regex;
extern crate reqwest;

mod formatter;
mod source;

use formatter::ServerFormatter;
use source::Source;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let sources = vec![Source {
        name: String::from("Harsh Shandilya's hosts list"),
        url: String::from("https://download.msfjarvis.website/adblock/hosts"),
    }];
    let write_file = File::create("adblock.list").unwrap();
    let mut writer = BufWriter::new(&write_file);
    match write!(&mut writer, "{}", sources[0].format().join("\n")) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    };
}
