use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use clap::Parser;
use maxminddb::{geoip2, MaxMindDBError};

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(author = "fukusuket")]
#[command(version = "0.1")]
#[command(about = "Lookup IP in the opened MaxMind DBs", long_about = None)]
struct Cli {
    /// MaxDB Path
    #[arg(short, long, value_name = "DB_PATH", default_value = "./db",)]
    db_path: String,

    /// IP file path
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli:Cli = Cli::parse();
    let db_base = Path::new("./").join(cli.db_path);
    let r1 = maxminddb::Reader::open_readfile(db_base.join("GeoLite2-ASN.mmdb")).unwrap();
    let r2 = maxminddb::Reader::open_readfile(db_base.join("GeoLite2-City.mmdb")).unwrap();
    let r3 = maxminddb::Reader::open_readfile(db_base.join("GeoLite2-Country.mmdb")).unwrap();

    for line in BufReader::new(File::open(cli.file.unwrap())?).lines() {
        let ip: IpAddr = line?.parse::<IpAddr>().unwrap();
        let asn: Result<geoip2::Asn, MaxMindDBError> = r1.lookup(ip);
        let city: Result<geoip2::City, MaxMindDBError> = r2.lookup(ip);
        let country: Result<geoip2::Country, MaxMindDBError> = r3.lookup(ip);

        let output_asn = if let Ok(asn) = asn {
            asn.autonomous_system_organization.unwrap_or("-")
        } else {
            "-"
        };

        let output_country = if let Ok(country_data) = country {
            if let Some(country) = country_data.country {
                let mut ret = "-";
                if let Some(name_tree) = country.names {
                    ret = name_tree.get("en").unwrap_or(&"-")
                }
                ret
            } else {
                "-"
            }
        } else {
            "-"
        };

        let output_city = if let Ok(city_data) = city {
            if let Some(city) = city_data.city {
                let mut ret = "n/-";
                if let Some(name_tree) = city.names {
                    ret = name_tree.get("en").unwrap_or(&"-")
                }
                ret
            } else {
                "-"
            }
        } else {
            "-"
        };

        let geo_data = format!("{output_asn},{output_country},{output_city}");
        println!("{},{}", ip, geo_data);
    }
    Ok(())
}