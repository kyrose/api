// Copyright 2015-2016 Intecture Developers. See the COPYRIGHT file at the
// top-level directory of this distribution and at
// https://intecture.io/COPYRIGHT.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

//! Dnf package provider

use command::{Command, CommandResult};
use error::{Error, Result};
use host::Host;
use regex::Regex;
use super::*;

pub struct Dnf;

impl Provider for Dnf {
    fn get_providers(&self) -> Providers {
        Providers::Dnf
    }

    fn is_active(&self, host: &mut Host) -> Result<bool> {
        let cmd = Command::new("which dnf");
        let result = try!(cmd.exec(host));

        Ok(result.exit_code == 0)
    }

    fn is_installed(&self, host: &mut Host, name: &str) -> Result<bool> {
        let cmd = Command::new("dnf list installed");
        let result = try!(cmd.exec(host));
        if result.exit_code != 0 {
            return Err(Error::Agent(result.stderr));
        }

        let arch = try!(needstr!(host.data() => "/telemetry/os/arch"));

        let re = try!(Regex::new(&format!("(?m)^{}\\.({}|noarch)\\s+", name, arch)));
        Ok(re.is_match(&result.stdout))
    }

    fn install(&self, host: &mut Host, name: &str) -> Result<CommandResult> {
        let cmd = Command::new(&format!("dnf -y install {}", name));
        cmd.exec(host)
    }

    fn uninstall(&self, host: &mut Host, name: &str) -> Result<CommandResult> {
        let cmd = Command::new(&format!("dnf -y remove {}", name));
        let result = try!(cmd.exec(host));

        if result.exit_code == 0 {
            let cmd = Command::new("dnf clean dbcache");
            try!(cmd.exec(host));
        }

        Ok(result)
    }
}
