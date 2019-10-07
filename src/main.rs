extern crate regex;
extern crate reqwest;

mod source;

use source::Source;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let sources = vec![Source {
        name: String::from("Harsh Shandilya's hosts list"),
        url: String::from("https://download.msfjarvis.website/adblock/hosts"),
    }];
    let write_file = File::create("adblock.list").unwrap();
    let mut contents = String::new();
    let mut writer = BufWriter::new(&write_file);
    for source in sources {
        contents.push_str(source.format_to_dnsmasq_server().join("\n").as_str())
    }
    match write!(&mut writer, "{}", contents) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    };
}
