use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

use std::thread;

mod parser;
use parser::sexp;

fn read_expr(stream: &mut TcpStream, v: &mut Vec<char>) -> sexp::Expr {
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

fn handle_client(stream: &mut TcpStream) {
    // let mut s: String = "".to_string();
    // stream.read_to_string(&mut s).unwrap();
    // println!("here: \"{}\"", s);
    let mut rest = vec![];
    loop {
        let arr = read_expr(stream, &mut rest);
        println!("{:?}", arr);
        
        // parser::sexp::parse_expr();
    }
}

fn eval(e: &Expr) {
    match e
}

fn listen() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6969")?;

    for stream in listener.incoming() {
        thread::spawn(move || handle_client(&mut stream.unwrap()));
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let a = parser::string::parse_string::<()>("\"\\t\\  \te\\\"st\"")
        .unwrap()
        .1;
    println!("parsed: `{}`", a);
    let s = "(false get (true \"abc\" 4) \"def\" (2 2))(";
    println!("{}", s);
    let b = parser::sexp::parse_expr(s)
        .unwrap();
    println!("parsed: {:?}", b);

    listen()?;

    Ok(())
}
