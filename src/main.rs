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
    let mut hosts = String::new();
    let mut dnsmasq_addrs = String::new();
    for source in sources {
        hosts.push_str(source.download_to_string().unwrap().as_str());
    }
    for line in hosts.lines() {
        let next = line.chars().next();
        if next == None || next.unwrap() == '#' {
            continue;
        }
        for section in line.split_whitespace() {
            if section == "0.0.0.0" || section == "127.0.0.1" {
                continue;
            }
            dnsmasq_addrs.push_str(format!("server=/{}/\n", section).as_str())
        }
    }
    let write_file = File::create("adblock.list").unwrap();
    let mut writer = BufWriter::new(&write_file);
    match write!(&mut writer, "{}", dnsmasq_addrs) {
        Ok(_) => {},
        Err(e) => println!("{}", e),
    };
}
