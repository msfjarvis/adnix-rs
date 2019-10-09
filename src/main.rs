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
        .arg(
            Arg::with_name("ipv4_addr")
            .long("address")
            .default_value("127.0.0.1")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("ipv6_addr")
            .long("v6address")
            .default_value("::1")
            .takes_value(true)
        )
        .get_matches();
    let ipv4_addr = matches.value_of("ipv4_addr").unwrap_or_default();
    let ipv6_addr = matches.value_of("ipv6_addr").unwrap_or_default();
    if let Some(val) = matches.value_of("formatter") {
        for source in sources.keys() {
            if let Some(s) = sources.get(source) {
                let raw_data = match val {
                    "dnsmasq" => s.format_to_dnsmasq(ipv4_addr, ipv6_addr),
                    "dnsmasq-server" => s.format_to_dnsmasq_server(),
                    "unbound" => s.format_to_unbound(ipv4_addr, ipv6_addr),
                    _ => panic!("Invalid formatter!"),
                };
                println!("INFO: {}: {} entries", source, raw_data.len());
                contents.push_str(raw_data.join("\n").as_str())
            }
        }
    }
    if let Some(val) = matches.value_of("output") {
        let write_file = File::create(val).unwrap();
        let mut writer = BufWriter::new(&write_file);
        match write!(&mut writer, "{}", contents) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        };
    } else {
        println!("{}", contents);
    }
}
