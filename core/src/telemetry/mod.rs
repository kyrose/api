// Copyright 2015-2017 Intecture Developers.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

//! Telemetry primitive.

mod providers;
mod serializable;

pub use self::providers::Macos;

use erased_serde::Serialize;
use errors::*;
use ExecutableProvider;
use host::Host;
use pnet::datalink::NetworkInterface;
use self::providers::MacosRemoteProvider;

pub trait TelemetryProvider {
    fn available(&Host) -> bool where Self: Sized;
    fn load(&Host) -> Result<Telemetry>;
}

#[derive(Serialize, Deserialize)]
pub enum RemoteProvider {
    Macos(MacosRemoteProvider)
}

impl <'de>ExecutableProvider<'de> for RemoteProvider {
    fn exec(&self, host: &Host) -> Result<Box<Serialize>> {
        match *self {
            RemoteProvider::Macos(ref p) => p.exec(host)
        }
    }
}

#[derive(Debug)]
pub struct Telemetry {
    pub cpu: Cpu,
    pub fs: Vec<FsMount>,
    pub hostname: String,
    pub memory: u64,
    pub net: Vec<NetworkInterface>,
    pub os: Os,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cpu {
    pub vendor: String,
    pub brand_string: String,
    pub cores: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FsMount {
    pub filesystem: String,
    pub mountpoint: String,
    pub size: u64,
    pub used: u64,
    pub available: u64,
    pub capacity: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Os {
    pub arch: String,
    pub family: String,
    pub platform: String,
    pub version_str: String,
    pub version_maj: u32,
    pub version_min: u32,
    pub version_patch: u32,
}

pub fn load(host: &Host) -> Result<Telemetry> {
    if Macos::available(host) {
        Macos::load(host)
    } else {
        Err(ErrorKind::ProviderUnavailable("Telemetry").into())
    }
}
