use crate::config;

use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, IpAddr};

struct Interface {
    port: u16,
    stream: TcpStream
}

impl Interface {
    pub fn connect(port: Option<u16>) -> io::Result<Self> {
        let port = port.unwrap_or(config::PORT);
        let stream = TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, port))?;
        
        Ok(Interface {
            port,
            stream
        })
    }
    
    fn get(&mut self, addr: String) {
        self.stream.write(addr.as_bytes());
    }
}
