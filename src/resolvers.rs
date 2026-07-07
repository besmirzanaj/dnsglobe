use std::net::IpAddr;
use std::sync::OnceLock;

use crate::sites::SiteProbe;

/// A resolver to query, either built-in or from the user's config file.
#[derive(Debug, Clone)]
pub struct Resolver {
    pub name: String,
    pub location: String,
    pub ip: IpAddr,
    /// (lat, lon) for the world-map view; None keeps the resolver off the map.
    pub coords: Option<(f64, f64)>,
    /// How to ask this resolver which anycast site answered, if it supports
    /// an identification query.
    pub probe: Option<SiteProbe>,
}

/// The resolver list for this run. Set once at startup (after the config file
/// is applied); falls back to the built-in list so tests and library callers
/// need no setup.
static ACTIVE: OnceLock<Vec<Resolver>> = OnceLock::new();

/// Install the resolver list for this run. Must be called before the first
/// `active()` call and at most once.
pub fn init(list: Vec<Resolver>) {
    ACTIVE
        .set(list)
        .expect("resolver list initialized more than once");
}

pub fn active() -> &'static [Resolver] {
    ACTIVE.get_or_init(defaults)
}

/// The built-in resolver list.
pub fn defaults() -> Vec<Resolver> {
    BUILTIN
        .iter()
        .map(|b| Resolver {
            name: b.name.into(),
            location: b.location.into(),
            ip: b.ip.parse().expect("built-in resolver IPs are valid"),
            coords: Some((b.lat, b.lon)),
            probe: b.probe,
        })
        .collect()
}

struct Builtin {
    name: &'static str,
    location: &'static str,
    ip: &'static str,
    lat: f64,
    lon: f64,
    probe: Option<SiteProbe>,
}

/// Public DNS resolvers spread across regions. Anycast networks are marked as
/// such — the answering node is the one nearest to you, so "location" is the
/// operator's home region, not necessarily where the query lands. Lat/lon are
/// therefore indicative, used for the world-map view.
///
/// Every entry has been verified to recursively resolve *uncommon* domains
/// for external clients, over both UDP and TCP 53. Answering example.com is
/// not enough: some resolvers (Bezeq IL, CNNIC CN) serve external clients
/// from cache only and refuse or time out on everything else, which makes
/// them useless for propagation checks.
/// Africa currently has no reliable open Do53 resolver (large ISPs there
/// refuse external queries); coverage comes from the anycast networks' POPs.
const BUILTIN: &[Builtin] = &[
    // Global anycast
    Builtin {
        name: "Google Public DNS",
        location: "Anycast",
        ip: "8.8.8.8",
        lat: 37.4,
        lon: -122.1,
        probe: Some(SiteProbe::Google),
    },
    Builtin {
        name: "Cloudflare",
        location: "Anycast",
        ip: "1.1.1.1",
        lat: 37.8,
        lon: -122.4,
        probe: Some(SiteProbe::Cloudflare),
    },
    Builtin {
        name: "Quad9",
        location: "CH/Any",
        ip: "9.9.9.9",
        lat: 47.4,
        lon: 8.5,
        probe: Some(SiteProbe::Quad9),
    },
    Builtin {
        name: "OpenDNS (Cisco)",
        location: "US/Any",
        ip: "208.67.222.222",
        lat: 33.9,
        lon: -118.2,
        probe: Some(SiteProbe::OpenDns),
    },
    Builtin {
        name: "CleanBrowsing",
        location: "Anycast",
        ip: "185.228.168.9",
        lat: 33.4,
        lon: -112.0,
        probe: Some(SiteProbe::ChIdServer),
    },
    // North America
    Builtin {
        name: "Level3",
        location: "US",
        ip: "4.2.2.2",
        lat: 39.7,
        lon: -105.0,
        probe: None,
    },
    Builtin {
        name: "Lumen (Qwest)",
        location: "US",
        ip: "205.171.3.66",
        lat: 40.4,
        lon: -104.0,
        probe: None,
    },
    Builtin {
        name: "Hurricane Electric",
        location: "US",
        ip: "74.82.42.42",
        lat: 37.6,
        lon: -122.0,
        probe: None,
    },
    Builtin {
        name: "DNSWatchGO",
        location: "US",
        ip: "54.174.40.213",
        lat: 38.9,
        lon: -77.4,
        probe: None,
    },
    Builtin {
        name: "Dyn (Oracle)",
        location: "US",
        ip: "216.146.35.35",
        lat: 43.0,
        lon: -71.5,
        probe: None,
    },
    Builtin {
        name: "Neustar UltraDNS",
        location: "US/Any",
        ip: "64.6.64.6",
        lat: 39.0,
        lon: -77.5,
        probe: Some(SiteProbe::ChIdServer),
    },
    Builtin {
        name: "Comodo Secure DNS",
        location: "US",
        ip: "8.26.56.26",
        lat: 40.9,
        lon: -74.2,
        probe: None,
    },
    Builtin {
        name: "FortiGuard",
        location: "US/Any",
        ip: "208.91.112.53",
        lat: 37.3,
        lon: -121.9,
        probe: None,
    },
    Builtin {
        name: "CIRA Canadian Shield",
        location: "CA",
        ip: "149.112.121.10",
        lat: 45.4,
        lon: -75.7,
        probe: None,
    },
    Builtin {
        name: "ControlD",
        location: "CA/Any",
        ip: "76.76.2.0",
        lat: 43.7,
        lon: -79.4,
        probe: None,
    },
    // Europe
    Builtin {
        name: "DNS4EU",
        location: "EU/Any",
        ip: "86.54.11.100",
        lat: 50.1,
        lon: 14.4,
        probe: None,
    },
    Builtin {
        name: "CZ.NIC ODVR",
        location: "CZ",
        ip: "193.17.47.1",
        lat: 49.9,
        lon: 15.3,
        probe: None,
    },
    Builtin {
        name: "AdGuard DNS",
        location: "EU/Any",
        ip: "94.140.14.14",
        lat: 34.7,
        lon: 33.0,
        probe: None,
    },
    Builtin {
        name: "Gcore DNS",
        location: "LU/Any",
        ip: "95.85.95.85",
        lat: 49.6,
        lon: 6.1,
        probe: None,
    },
    Builtin {
        name: "DNS.SB",
        location: "DE/Any",
        ip: "185.222.222.222",
        lat: 50.1,
        lon: 8.7,
        probe: None,
    },
    Builtin {
        name: "DNS for Family",
        location: "DE",
        ip: "94.130.180.225",
        lat: 50.5,
        lon: 12.3,
        probe: None,
    },
    Builtin {
        name: "LibreDNS",
        location: "DE",
        ip: "88.198.92.222",
        lat: 49.5,
        lon: 11.1,
        probe: None,
    },
    Builtin {
        name: "Surfshark DNS",
        location: "LT",
        ip: "194.169.169.169",
        lat: 54.7,
        lon: 25.3,
        probe: None,
    },
    // Russia / Middle East
    Builtin {
        name: "SafeDNS",
        location: "RU",
        ip: "195.46.39.39",
        lat: 55.8,
        lon: 37.6,
        probe: None,
    },
    Builtin {
        name: "Yandex DNS",
        location: "RU",
        ip: "77.88.8.8",
        lat: 55.6,
        lon: 37.9,
        probe: None,
    },
    Builtin {
        name: "Comss.one",
        location: "RU",
        ip: "83.220.169.155",
        lat: 56.3,
        lon: 38.1,
        probe: None,
    },
    Builtin {
        name: "Shecan",
        location: "IR",
        ip: "178.22.122.100",
        lat: 35.7,
        lon: 51.4,
        probe: None,
    },
    // East Asia
    Builtin {
        name: "114DNS",
        location: "CN",
        ip: "114.114.114.114",
        lat: 32.1,
        lon: 118.8,
        probe: None,
    },
    Builtin {
        name: "AliDNS",
        location: "CN",
        ip: "223.5.5.5",
        lat: 30.3,
        lon: 120.2,
        probe: None,
    },
    Builtin {
        name: "DNSPod (Tencent)",
        location: "CN",
        ip: "119.29.29.29",
        lat: 22.5,
        lon: 114.1,
        probe: None,
    },
    Builtin {
        name: "Baidu DNS",
        location: "CN",
        ip: "180.76.76.76",
        lat: 39.9,
        lon: 116.4,
        probe: None,
    },
    Builtin {
        name: "ByteDance Public DNS",
        location: "CN",
        ip: "180.184.1.1",
        lat: 40.0,
        lon: 116.3,
        probe: None,
    },
    Builtin {
        name: "360 Secure DNS",
        location: "CN",
        ip: "101.226.4.6",
        lat: 31.2,
        lon: 121.5,
        probe: None,
    },
    Builtin {
        name: "KT (Kornet)",
        location: "KR",
        ip: "168.126.63.1",
        lat: 37.6,
        lon: 127.0,
        probe: None,
    },
    Builtin {
        name: "LG U+",
        location: "KR",
        ip: "164.124.101.2",
        lat: 36.5,
        lon: 127.9,
        probe: None,
    },
    Builtin {
        name: "HiNet (Chunghwa)",
        location: "TW",
        ip: "168.95.1.1",
        lat: 25.0,
        lon: 121.6,
        probe: None,
    },
    // Southern hemisphere
    Builtin {
        name: "Telstra",
        location: "AU",
        ip: "139.130.4.4",
        lat: -33.9,
        lon: 151.2,
        probe: None,
    },
    Builtin {
        name: "SafeSurfer",
        location: "NZ",
        ip: "104.197.28.121",
        lat: -36.8,
        lon: 174.8,
        probe: None,
    },
    Builtin {
        name: "UOL",
        location: "BR",
        ip: "200.221.11.100",
        lat: -23.5,
        lon: -46.6,
        probe: None,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_ips_parse_and_are_unique() {
        let defaults = defaults(); // parses every IP; panics on a bad entry
        let mut ips: Vec<IpAddr> = defaults.iter().map(|r| r.ip).collect();
        ips.sort();
        ips.dedup();
        assert_eq!(ips.len(), defaults.len());
    }
}
