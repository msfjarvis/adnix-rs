# adnix-rs

[![GitHub workflow](https://github.com/msfjarvis/adnix-rs/workflows/Test%20Rust%20project/badge.svg)](https://github.com/msfjarvis/adnix-rs/actions)

Rust reimplementation of [sniner/adnix](https://github.com/sniner/adnix) for educational purposes.

## Installation

adnix is not available on [crates.io](https://crates.io) as of now, but you can install it through cargo and this git repository.

```shell
$ cargo install --git https://github.com/msfjarvis/adnix-rs --bin adnix
    Updating git repository `https://github.com/msfjarvis/adnix-rs`
  Installing adnix v0.1.0 (https://github.com/msfjarvis/adnix-rs#d90134fb)
... (truncated)
  Installing /home/msfjarvis/.cargo/bin/adnix
   Installed package `adnix v0.1.0 (https://github.com/msfjarvis/adnix-rs#d90134fb)` (executable `adnix`)
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
