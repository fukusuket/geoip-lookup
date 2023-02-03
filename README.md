# geoip-lookup

- Lookup the IP address in the opened MaxMind DB

## Usage
```
Usage: geoip-lookup [OPTIONS] --file <FILE>

Options:
  -d, --db-path <DIR>  MaxMind DBs directory path(GeoLite2-ASN.mmdb/GeoLite2-City.mmdb/GeoLite2-Country.mmdb) [default: ./db]
  -f, --file <FILE>    IP file path(only one column)
  -h, --help           Print help
  -V, --version        Print version
```

## Pre-requisites
1. [Install Rust.](https://www.rust-lang.org/tools/install)
   - [You need c++ build tool(on windows)](https://docs.microsoft.com/ja-jp/windows/dev-environment/rust/setup).
2. Get maxmind db following three files from [MAXMIND Web site](https://www.maxmind.com/en/home).(Login account required)
   - GeoLite2-ASN.mmdb
   - GeoLite2-City.mmdb
   - GeoLite2-Country.mmdb
3. Put the above 3 files in any directory

## How to use(from release)
- TODO

## How to use(from source)


## Input

A file with one column of IP addresses as below

```
192.168.0.1
192.168.0.2
192.168.0.3
...
```

## Output

Then IPs are enriched by MaxMind DB

```
192.168.0.1,Hogehoge Internet Ltd,United Kingdom,Kendal
192.168.0.2,Hello-CAROLINAS,United States,Jacksonville
192.168.0.3,ADB01,Jamaica,Kingston
```


## Acknowledgements
- [maxminddb-rust](https://github.com/oschwald/maxminddb-rust) library by @oschwald
- [hayabusa](https://github.com/Yamato-Security/hayabusa) by @Yamato-Security, @hitenkoku
