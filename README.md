# geoip-lookup

- Lookup the IP address in the opened MaxMind DB

## Usage
```
Usage: geoip-lookup [OPTIONS]

Options:
  -d, --db-path <DB_PATH>  MaxDB Path [default: ./db]
  -f, --file <FILE>        IP file path
  -h, --help               Print help
  -V, --version            Print version

```

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
- [hayabusa](https://github.com/Yamato-Security/hayabusa) by @Yamato-Security
