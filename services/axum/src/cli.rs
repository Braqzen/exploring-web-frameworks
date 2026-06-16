use eyre::{eyre, Result};
use std::{net::SocketAddr, str::FromStr};

pub struct Cli {
    pub socket: Option<SocketAddr>,
}

impl Cli {
    pub fn parse() -> Result<Self> {
        // Skip binary name
        let mut args = std::env::args().skip(1);

        let mut socket: Option<SocketAddr> = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--socket" | "-s" => {
                    if let Some(val) = args.next() {
                        socket = Some(SocketAddr::from_str(&val)?);
                    } else {
                        return Err(eyre!("--socket requires a value"));
                    }
                }
                _ => {}
            }
        }

        Ok(Self { socket })
    }
}
