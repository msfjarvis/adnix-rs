extern crate clap;
extern crate regex;
extern crate ureq;

mod formatters;
mod source;

use clap::{load_yaml, App};
use source::Source;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    process,
};

fn main() {
    let yaml = load_yaml!("adnix.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let sources: HashMap<String, Source> = if matches.is_present("sources_file") {
        parse_sources_config_file(&matches.value_of("sources_file").unwrap())
    } else {
        let mut list: HashMap<String, Source> = HashMap::new();
        list.insert(
            String::from("Harsh Shandilya's hosts list"),
            Source {
                url: String::from("https://dl.msfjarvis.dev/adblock/hosts"),
            },
        );
        list
    };

    let mut contents = String::new();
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
            Err(e) => eprintln!("{}", e),
        };
    } else {
        println!("{}", contents);
    }
}

fn parse_sources_config_file(filepath: &str) -> HashMap<String, Source> {
    let mut list: HashMap<String, Source> = HashMap::new();
    if let Ok(file) = File::open(filepath) {
        let lines = BufReader::new(file).lines();
        for line in lines {
            let content = line.unwrap_or_else(|err| {
                eprintln!("Problem parsing lines: {}", err);
                process::exit(1);
            });
            let vec: Vec<&str> = content.split('|').collect();
            list.insert(
                vec[0].to_owned(),
                Source {
                    url: vec[1].to_owned(),
                },
            );
        }
    } else {
        eprintln!("Problem opening file: {}", filepath);
    };
    list
}

#[cfg(test)]
mod tests {
    use super::parse_sources_config_file;

    #[test]
    fn test_parse_sources_config_file() {
        let source_config = parse_sources_config_file("test_data/sample_config");
        assert!(source_config.contains_key("StevenBlack"));
        assert!(source_config.get("StevenBlack").is_some());
        assert!(source_config.get("StevenBlack").unwrap().url == String::from("https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts"));
    }
}
