use regex::Regex;

const REGEX_PATTERN: &str = r"^\s*(\d+\.\d+\.\d+\.\d+)\s+([\w\-\.]+)";
// I don't fully understand this and would appreciate help :)
const LOCALHOST_ADDRS: &[&str] = &[
    "localhost",
    "local",
    "localhost.localdomain",
    "broadcasthost",
];

pub fn format_to_unbound(raw_hosts: String, ipv4_addr: &str, ipv6_addr: &str) -> Vec<String> {
    let mut output: Vec<String> = vec![String::from("server:")];
    let re = Regex::new(REGEX_PATTERN).unwrap();
    for line in raw_hosts.lines() {
        if !re.is_match(line) {
            continue;
        }
        for cap in re.captures_iter(line) {
            if !LOCALHOST_ADDRS.contains(&&cap[2]) {
                output.push(format!("  local-zone: {} redirect", &cap[2]));
                output.push(format!("  local-zone: {} A {}", &cap[2], ipv4_addr));
                output.push(format!("  local-zone: {} AAAAA {}", &cap[2], ipv6_addr))
            }
        }
    }
    output
}

pub fn format_to_dnsmasq_server(raw_hosts: String) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    let re = Regex::new(REGEX_PATTERN).unwrap();
    for line in raw_hosts.lines() {
        if !re.is_match(line) {
            continue;
        }
        for cap in re.captures_iter(line) {
            if !LOCALHOST_ADDRS.contains(&&cap[2]) {
                output.push(format!("server=/{}/", &cap[2]))
            }
        }
    }
    output
}

pub fn format_to_dnsmasq(raw_hosts: String, ipv4_addr: &str, ipv6_addr: &str) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    let re = Regex::new(REGEX_PATTERN).unwrap();
    for line in raw_hosts.lines() {
        if !re.is_match(line) {
            continue;
        }
        for cap in re.captures_iter(line) {
            if !LOCALHOST_ADDRS.contains(&&cap[2]) {
                output.push(format!("address=/{}/{}", &cap[2], ipv4_addr));
                output.push(format!("address=/{}/{}", &cap[2], ipv6_addr))
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    fn emit_hosts_file() -> String {
        std::fs::read_to_string("test_data/hosts").unwrap()
    }

    #[test]
    fn test_dnsmasq_formatter() {
        let results = format_to_dnsmasq(emit_hosts_file(), "127.0.0.1", "::1");
        assert!(results.contains(&String::from("address=/adserver.abv.bg/127.0.0.1")));
        assert!(results.contains(&String::from("address=/adserver.abv.bg/::1")));
        assert!(!results.contains(&String::from("# Leading comment test 0.0.0.0 fr.a2dfp.net")));
        assert!(!results.contains(&String::from("# Normal comment")));
        assert!(!results.contains(&String::from("address=/localhost/127.0.01")));
    }
}
