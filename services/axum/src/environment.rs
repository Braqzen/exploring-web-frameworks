use crate::cli::Cli;
use eyre::{Result, eyre};
use std::{env::var, net::SocketAddr, str::FromStr};

pub struct Environment {
    pub socket: SocketAddr,
}

impl Environment {
    pub fn new(args: &Cli) -> Result<Self> {
        let socket = args
            .socket
            .or_else(|| {
                var("SOCKET")
                    .ok()
                    .and_then(|socket| SocketAddr::from_str(&socket).ok())
            })
            .ok_or_else(|| eyre!("Failed to parse SOCKET"))?;

        Ok(Self { socket })
    }
}
