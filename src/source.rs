use regex::Regex;
use std::error::Error;

pub struct Source {
    pub url: String,
}

impl Source {
    // I don't fully understand this and would appreciate help :)
    const LOCALHOST_ADDRS: &'static [&'static str] = &[
        "localhost",
        "local",
        "localhost.localdomain",
        "broadcasthost",
    ];
    const REGEX_PATTERN: &'static str = r"^\s*(\d+\.\d+\.\d+\.\d+)\s+([\w\-\.]+)";
    const IPV4: &'static str = "127.0.0.1";
    const IPV6: &'static str = "::1";

    pub fn download_to_string(&self) -> Result<String, Box<dyn Error>> {
        let req = reqwest::get(self.url.as_str())?.text()?;
        Ok(req)
    }

    pub fn format_to_dnsmasq(&self) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        let re = Regex::new(Self::REGEX_PATTERN).unwrap();
        let raw_hosts = self.download_to_string().unwrap();
        for line in raw_hosts.lines() {
            if !re.is_match(line) {
                continue;
            }
            for cap in re.captures_iter(line) {
                if !Self::LOCALHOST_ADDRS.contains(&&cap[2]) {
                    output.push(format!("address=/{}/{}", &cap[2], Self::IPV4));
                    output.push(format!("address=/{}/{}", &cap[2], Self::IPV6))
                }
            }
        }
        output
    }

    pub fn format_to_dnsmasq_server(&self) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        let re = Regex::new(Self::REGEX_PATTERN).unwrap();
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

    pub fn format_to_unbound(&self) -> Vec<String> {
        let mut output: Vec<String> = vec![String::from("server:")];
        let re = Regex::new(Self::REGEX_PATTERN).unwrap();
        let raw_hosts = self.download_to_string().unwrap();
        for line in raw_hosts.lines() {
            if !re.is_match(line) {
                continue;
            }
            for cap in re.captures_iter(line) {
                if !Self::LOCALHOST_ADDRS.contains(&&cap[2]) {
                    output.push(format!("  local-zone: {} redirect", &cap[2]));
                    output.push(format!("  local-zone: {} A {}", &cap[2], Self::IPV4));
                    output.push(format!("  local-zone: {} AAAAA {}", &cap[2], Self::IPV6))
                }
            }
        }
        output
    }
}
