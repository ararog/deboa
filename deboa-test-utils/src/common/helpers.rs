#![allow(dead_code)]
use once_cell::sync::OnceCell;
use std::{collections::HashSet, sync::Mutex};
use url::Url;

pub const CA_CERT: &[u8] = include_bytes!("../../../certs/ca.der");
// pub const CA_CERT_PEM: &[u8] = include_bytes!("../../../certs/ca.crt");

pub const SERVER_CERT: &[u8] = include_bytes!("../../../certs/server.der");
pub const SERVER_KEY: &[u8] = include_bytes!("../../../certs/server.key.der");

// pub const IP6_SERVER_CERT: &[u8] = include_bytes!("../../../certs/ip6-server.der");
// pub const IP6_SERVER_KEY: &[u8] = include_bytes!("../../../certs/ip6-server.key.der");

// pub const SERVER_CERT_PEM: &[u8] = include_bytes!("../../../certs/server.crt");
// pub const SERVER_KEY_PEM: &[u8] = include_bytes!("../../../certs/server.key");

pub const CLIENT_CERT: &[u8] = include_bytes!("../../../certs/client.der");
pub const CLIENT_KEY: &[u8] = include_bytes!("../../../certs/client.key.der");

pub const CLIENT_CERT_PEM: &[u8] = include_bytes!("../../../certs/client.crt");
pub const CLIENT_KEY_PEM: &[u8] = include_bytes!("../../../certs/client.key");
pub const CLIENT_P12: &[u8] = include_bytes!("../../../certs/client.p12");

static PORTS_IN_USE: OnceCell<Mutex<HashSet<u16>>> = OnceCell::new();

pub fn fake_url() -> Url {
    Url::parse("https://httpbin.org/get").unwrap()
}
