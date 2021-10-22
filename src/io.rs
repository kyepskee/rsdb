use std::io::Read;
use std::net::TcpStream;

use sexplib::{parser, sexp};

pub fn read_expr(stream: &mut TcpStream, v: &mut Vec<char>) -> sexp::Expr {
    let mut buf = [0u8; 256];
    loop {
        let n = stream.read(&mut buf).unwrap();
        for i in 0..n {
            v.push(buf[i] as char)
        }
        let s: &str = &v.iter().collect::<String>();
        if let Ok((rest, parsed)) = parser::sexp::parse_expr(s) {
            *v = rest.chars().collect::<Vec<_>>();
            return parsed;
        }
        // v.extend_from_slice(&(buf.map(|u| u as char)));
    }
}
