use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

pub const LOCATION: &str = "/tmp/pppoe.ip_config";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpConfig {
    pub addr: Ipv4Addr,
    pub dns1: Ipv4Addr,
    pub dns2: Ipv4Addr,
    pub rtr: Ipv4Addr,
}

impl Default for IpConfig {
    fn default() -> Self {
        Self {
            addr: Ipv4Addr::UNSPECIFIED,
            dns1: Ipv4Addr::UNSPECIFIED,
            dns2: Ipv4Addr::UNSPECIFIED,
            rtr: Ipv4Addr::UNSPECIFIED,
        }
    }
}
