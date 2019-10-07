use crate::Source;
use regex::Regex;

pub trait Formatter {
    // I don't fully understand this and would appreciate help :)
    const LOCALHOST_ADDRS: &'static [&'static str] = &[
        "localhost",
        "local",
        "localhost.localdomain",
        "broadcasthost",
    ];
    const REGEX_PATTERN: &'static str = r"^\s*(\d+\.\d+\.\d+\.\d+)\s+([\w\-\.]+)";
    fn format(&self) -> Vec<String>;
}

impl Formatter for Source {
    fn format(&self) -> Vec<String> {
        let re = Regex::new(Self::REGEX_PATTERN).unwrap();
        let mut output: Vec<String> = vec![];
        let raw_hosts = self.download_to_string().unwrap();
        for line in raw_hosts.lines() {
            if !re.is_match(line) {
                continue;
            }
            for cap in re.captures_iter(line) {
                if !Self::LOCALHOST_ADDRS.contains(&&cap[2]) {
                    output.push(format!("server=/{}/", &cap[2]))
                }
            }
        }
        output
    }
}
