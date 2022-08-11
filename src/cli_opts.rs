use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about = "CLI tool to convert ad blocking hosts files into DNSMasq or Unbound configuration files"
)]
pub(crate) struct Opts {
    /// Output file
    #[clap(long, short, action)]
    pub(crate) output: Option<String>,

    /// Formatter
    #[clap(value_enum, long, short, default_value_t = FormatterOpt::Dnsmasq, action)]
    pub(crate) formatter: FormatterOpt,

    /// IPv4 address
    #[clap(long = "address", default_value = "127.0.0.1", action)]
    pub(crate) ipv4_address: String,

    /// IPv6 address
    #[clap(long = "v6address", default_value = "::1", action)]
    pub(crate) ipv6_address: String,

    /// File to read "name|source url" mappings from
    #[clap(long, short, action)]
    pub(crate) sources_file: String,
}

#[derive(ValueEnum, Clone, Debug)]
pub(crate) enum FormatterOpt {
    Dnsmasq,
    DnsmasqServer,
    Unbound,
}

#[cfg(test)]
mod test {
    use super::Opts;

    #[test]
    fn cli_assert() {
        <Opts as clap::CommandFactory>::command().debug_assert()
    }
}
