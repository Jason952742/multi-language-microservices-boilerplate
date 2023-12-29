use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Deserialize, Serialize)]
pub struct ConsulOption {
    pub addr: String,
    pub timeout_sec: u64,
    pub protocol: String,
}

impl Default for ConsulOption {
    fn default() -> Self {
        Self {
            addr: String::from("127.0.0.1:8500"),
            timeout_sec: 1u64,
            protocol: "http".to_string(),
        }
    }
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct Registration {
    pub name: String,
    pub id: String,
    pub tags: Vec<String>,
    pub address: String,
    pub port: i32,
    pub check: Check
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct Check {
    pub grpc: String,
    pub interval: String,
    pub timeout: String,
}

impl Registration {
    pub fn new(name: ServiceName, id: &str, tags: Vec<&str>, addr: &str, port: i32) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
            address: addr.to_string(),
            port,
            check: Check {
                grpc: format!("host.docker.internal:{}", port),
                interval: "10s".to_string(),
                timeout: "5s".to_string(),
            },
        }
    }

    pub fn simple_with_tags(name: ServiceName, tags: Vec<&str>, addr: &str, port: i32) -> Self {
        let id: &str = &format!("{}-{}", name, port);
        Self::new(name, id, tags, addr, port)
    }

    pub fn simple(name: ServiceName, addr: &str, port: i32) -> Self {
        Self::simple_with_tags(name, vec![], addr, port)
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthService {
    #[serde(rename = "ID")]
    pub id: String,
    pub service: String,
    pub tags: Vec<String>,
    pub address: String,
    pub port: i32,
    pub datacenter: String,
}

pub type Services = HashMap<String, HealthService>;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum Filter {
    Service(String),
    ID(String),
}


#[derive(Debug, Deserialize, Serialize, EnumString, strum_macros::Display)]
pub enum ServiceName {
    #[strum(serialize = "mu_referral")]
    MuReferral,
    #[strum(serialize = "mu_member")]
    MuMember,
}
