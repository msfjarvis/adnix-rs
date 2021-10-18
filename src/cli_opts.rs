use clap::{ArgEnum, Parser};

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about = "CLI tool to convert ad blocking hosts files into DNSMasq or Unbound configuration files"
)]
pub(crate) struct Opts {
    /// Output file
    #[clap(long, short)]
    pub(crate) output: Option<String>,

    /// Formatter
    #[clap(arg_enum, long, short, default_value = "dnsmasq")]
    pub(crate) formatter: FormatterOpt,

    /// IPv4 address
    #[clap(long = "address", default_value = "127.0.0.1")]
    pub(crate) ipv4_address: String,

    /// IPv6 address
    #[clap(long = "v6address", default_value = "::1")]
    pub(crate) ipv6_address: String,

    /// File to read "name|source url" mappings from
    #[clap(long, short)]
    pub(crate) sources_file: String,
}

#[derive(ArgEnum, Clone, Debug, Parser)]
pub(crate) enum FormatterOpt {
    Dnsmasq,
    DnsmasqServer,
    Unbound,
}
