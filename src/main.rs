extern crate regex;
extern crate reqwest;

mod source;

use regex::Regex;
use source::Source;
use std::fs::File;
use std::io::{BufWriter, Write};

// I don't fully understand this and would appreciate help :)
const LOCALHOST_ADDRS: &[&str] = &[
    "localhost",
    "local",
    "localhost.localdomain",
    "broadcasthost",
];

fn main() {
    let sources = vec![Source {
        name: String::from("Harsh Shandilya's hosts list"),
        url: String::from("https://download.msfjarvis.website/adblock/hosts"),
    }];
    let re = Regex::new(r"^\s*(\d+\.\d+\.\d+\.\d+)\s+([\w\-\.]+)").unwrap();
    let mut hosts = String::new();
    let mut dnsmasq_addrs = String::new();
    for source in sources {
        hosts.push_str(source.download_to_string().unwrap().as_str());
    }
    for line in hosts.lines() {
        if !re.is_match(line) {
            continue;
        }
        for cap in re.captures_iter(line) {
            if !LOCALHOST_ADDRS.contains(&&cap[2]) {
                dnsmasq_addrs.push_str(format!("server=/{}/\n", &cap[2]).as_str())
            }
        }
    }
    let write_file = File::create("adblock.list").unwrap();
    let mut writer = BufWriter::new(&write_file);
    match write!(&mut writer, "{}", dnsmasq_addrs) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    };
}
