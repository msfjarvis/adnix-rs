extern crate clap;
extern crate regex;
extern crate reqwest;

mod source;

use clap::{App, Arg};
use source::Source;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let sources = vec![Source {
        name: String::from("Harsh Shandilya's hosts list"),
        url: String::from("https://download.msfjarvis.website/adblock/hosts"),
    }];
    let matches = App::new("adnix-rs").version("0.1.0")
        .author("Harsh Shandilya <msfjarvis@gmail.com>")
        .about("CLI tool to convert ad blocking hosts files into DNSMasq or Unbound configuration files")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("file")
                .value_name("OUTPUT")
                .help("Output file")
                .default_value("adblock.list")
                .takes_value(true),
        )
        .get_matches();
    let write_file = File::create(matches.value_of("output").unwrap_or_default()).unwrap();
    let mut contents = String::new();
    let mut writer = BufWriter::new(&write_file);
    for source in sources {
        contents.push_str(source.format_to_dnsmasq().join("\n").as_str())
    }
    match write!(&mut writer, "{}", contents) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    };
}
