use crate::config;

use sexplib::sexp::Expr;

use std::io;
use std::io::prelude::*;
use std::net::{IpAddr, TcpListener, TcpStream};

use crate::io::read_expr;

struct Interface {
    port: u16,
    stream: TcpStream,
    buf: Vec<char>,
}

impl Interface {
    pub fn connect(port: Option<u16>) -> io::Result<Self> {
        let port = port.unwrap_or(config::PORT);
        let stream = TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, port))?;

        Ok(Interface {
            port,
            stream,
            buf: vec![],
        })
    }

    fn query_expr(&mut self, q: &Expr) -> Expr {
        q.to_string();
        read_expr(&mut self.stream, &mut self.buf)
    }

    fn get(&mut self, addr: String) {
        self.stream.write(addr.as_bytes());
    }
}
