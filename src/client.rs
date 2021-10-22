use crate::config;

use sexplib::sexp::Expr;

use std::io;
use std::io::prelude::*;
use std::net::{IpAddr, TcpListener, TcpStream};

use crate::io::read_expr;

pub struct Interface {
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
    
    fn read_expr(&mut self) -> Expr {
        read_expr(&mut self.stream, &mut self.buf)
    }
    
    pub fn set(&mut self, addr: String, val: Expr) -> io::Result<usize>{
        self.stream.write(format!("(get {} {})", addr, val.to_string()).as_bytes())
    }
    
    pub fn get(&mut self, addr: String) -> Expr {
        self.stream.write(format!("(get {})", addr).as_bytes()).unwrap();
        self.read_expr()
    }
}
