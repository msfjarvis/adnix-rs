use crate::formatters;

pub struct Source {
    pub url: String,
}

impl Source {
    pub fn download_to_string(&self) -> Result<String, std::io::Error> {
        ureq::get(self.url.as_str()).call().into_string()
    }

    pub fn format_to_dnsmasq(&self, ipv4_addr: &str, ipv6_addr: &str) -> Vec<String> {
        let raw_hosts = self.download_to_string().unwrap();
        formatters::format_to_dnsmasq(raw_hosts, ipv4_addr, ipv6_addr)
    }

    pub fn format_to_dnsmasq_server(&self) -> Vec<String> {
        let raw_hosts = self.download_to_string().unwrap();
        formatters::format_to_dnsmasq_server(raw_hosts)
    }

    pub fn format_to_unbound(&self, ipv4_addr: &str, ipv6_addr: &str) -> Vec<String> {
        let raw_hosts = self.download_to_string().unwrap();
        formatters::format_to_unbound(raw_hosts, ipv4_addr, ipv6_addr)
    }
}
