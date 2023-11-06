use std::net::{Ipv4Addr, Ipv6Addr};

use serde::{Deserialize, Serialize};

pub const LOCATION: &str = "/data/pppoe.ip_config";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ipv4Config {
    pub addr: Ipv4Addr,
    pub dns1: Ipv4Addr,
    pub dns2: Ipv4Addr,
}

impl Default for Ipv4Config {
    fn default() -> Self {
        Self {
            addr: Ipv4Addr::UNSPECIFIED,
            dns1: Ipv4Addr::UNSPECIFIED,
            dns2: Ipv4Addr::UNSPECIFIED,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ipv6Config {
    pub laddr: Ipv6Addr,
    pub raddr: Ipv6Addr,
}

impl Default for Ipv6Config {
    fn default() -> Self {
        Self {
            laddr: Ipv6Addr::UNSPECIFIED,
            raddr: Ipv6Addr::UNSPECIFIED,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct DsConfig {
    pub v4: Option<Ipv4Config>,
    pub v6: Option<Ipv6Config>,
}
