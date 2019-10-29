# adnix-rs

[![GitHub workflow](https://github.com/msfjarvis/adnix-rs/workflows/CI%20Test/badge.svg)](https://github.com/msfjarvis/adnix-rs/actions)
[![Version info](https://img.shields.io/crates/v/adnix.svg)](https://crates.io/crates/adnix)

Rust reimplementation of [sniner/adnix](https://github.com/sniner/adnix) for educational purposes.

## Installation

adnix is available on [crates.io](https://crates.io/crates/adnix) and you can install it through cargo.

```shell
cargo install adnix
```

## Usage

```shell
USAGE:
    adnix [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --formatter <STRING>       Formatter [default: dnsmasq]  [possible values: dnsmasq, dnsmasq-server, unbound]
        --address <ADDRESS>        IPv4 address [default: 127.0.0.1]
        --v6address <ADDRESS>      IPv6 address [default: ::1]
    -o, --output <OUTPUT>          Output file
    -s, --sources_file <STRING>    File to read "name|source url" mappings from
```

Sample sources file for use with adnix.

```plaintext
Yoyo|http://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&showintro=0&mimetype=plaintext
Malware Domain List|http://www.malwaredomainlist.com/hostslist/hosts.txt
```
