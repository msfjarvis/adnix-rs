mod cli_opts;
mod formatters;
mod source;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use clap::Parser;
use color_eyre::Result;

use crate::{
    cli_opts::{FormatterOpt, Opts},
    source::Source,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let opts: Opts = Opts::parse();

    let sources = parse_sources_config_file(&opts.sources_file);

    let mut contents = String::new();
    let ipv4_addr = opts.ipv4_address;
    let ipv6_addr = opts.ipv6_address;
    let formatter = opts.formatter;
    for source in sources.keys() {
        if let Some(s) = sources.get(source) {
            let raw_data = match formatter {
                FormatterOpt::Dnsmasq => s.format_to_dnsmasq(&ipv4_addr, &ipv6_addr),
                FormatterOpt::DnsmasqServer => s.format_to_dnsmasq_server(),
                FormatterOpt::Unbound => s.format_to_unbound(&ipv4_addr, &ipv6_addr),
            };
            println!("INFO: {}: {} entries", source, raw_data.len());
            contents.push_str(raw_data.join("\n").as_str());
        }
    }
    if let Some(val) = opts.output {
        let write_file = File::create(val).unwrap();
        let mut writer = BufWriter::new(&write_file);
        match write!(&mut writer, "{}", contents) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        };
    } else {
        println!("{}", contents);
    };
    Ok(())
}

fn parse_sources_config_file(filepath: &str) -> HashMap<String, Source> {
    let mut list: HashMap<String, Source> = HashMap::new();
    if let Ok(file) = File::open(filepath) {
        BufReader::new(file)
            .lines()
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap())
            .for_each(|line| {
                let vec: Vec<&str> = line.split('|').collect();
                list.insert(
                    vec[0].to_owned(),
                    Source {
                        url: vec[1].to_owned(),
                    },
                );
            });
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
        assert_eq!(
            source_config.get("StevenBlack").unwrap().url,
            *"https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts",
        );
    }
}
