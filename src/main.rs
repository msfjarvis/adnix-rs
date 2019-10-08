extern crate clap;
extern crate regex;
extern crate reqwest;

mod source;

use clap::{App, Arg};
use source::Source;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let mut contents = String::new();
    let mut sources: HashMap<&str, Source> = HashMap::new();
    sources.insert(
        "Harsh Shandilya's hosts list",
        Source {
            url: String::from("https://download.msfjarvis.website/adblock/hosts"),
        },
    );
    let matches = App::new("adnix-rs").version("0.1.0")
        .author("Harsh Shandilya <msfjarvis@gmail.com>")
        .about("CLI tool to convert ad blocking hosts files into DNSMasq or Unbound configuration files")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("file")
                .value_name("OUTPUT")
                .help("Output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("formatter")
                .short("f")
                .long("formatter")
                .default_value("dnsmasq")
                .takes_value(true)
                .possible_values(&["dnsmasq", "dnsmasq-server", "unbound"])
        )
        .get_matches();
    match matches.value_of("formatter") {
        Some(val) => {
            for source in sources.keys() {
                if let Some(s) = sources.get(source) {
                    let raw_data = match val {
                        "dnsmasq" => s.format_to_dnsmasq(),
                        "dnsmasq-server" => s.format_to_dnsmasq_server(),
                        "unbound" => s.format_to_unbound(),
                        _ => panic!("Invalid formatter!"),
                    };
                    println!("INFO: {}: {} entries", source, raw_data.len());
                    contents.push_str(raw_data.join("\n").as_str())
                }
            }
        }
        None => {}
    }
    match matches.value_of("output") {
        Some(val) => {
            let write_file = File::create(val).unwrap();
            let mut writer = BufWriter::new(&write_file);
            match write!(&mut writer, "{}", contents) {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            };
        }
        None => {
            println!("{}", contents);
        }
    };
}
