#![feature(box_syntax)]
#![feature(box_patterns)]

use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use std::sync::Arc;

use std::thread;

mod parser;
use parser::pp;
mod db;
use db::Database;
mod sexp;
use sexp::{Expr, Atom};

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

fn handle_client(stream: &mut TcpStream, db: Arc<Database>) {
    // let mut s: String = "".to_string();
    // stream.read_to_string(&mut s).unwrap();
    // println!("here: \"{}\"", s);
    let mut rest = vec![];
    loop {
        let e = read_expr(stream, &mut rest);
        println!("{:?}", e);
        eval(&e, stream, &db)
    }
}

fn eval(e: &Expr, s: &mut TcpStream, db: &Arc<Database>) {
    if let Expr::List(box x) = e {
        if x[0] == Expr::Atom(Atom::Sym(String::from("get"))) {
            let addr = x[1]
                .atom().expect("Expected first argument to be atom")
                .str().expect("Expected fisrt argument to be string");
            let res = pp::pp_atom(&db.get(addr));
            s.write(res.as_bytes()).unwrap();
        }
        if x[0] == Expr::Atom(Atom::Sym(String::from("get"))) {
            let addr = x[1]
                .atom().expect("Expected first argument to be atom")
                .str().expect("Expected fisrt argument to be string");
            
            let val = x[2].atom().expect("Expected second argument to be atom"); 
            db.set(addr, val);
        }
    } else {
        
    }
}

fn listen(db: Arc<Database>) -> io::Result<()> {
    use std::net::SocketAddr;
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 6969)))?;

    for stream in listener.incoming() {
        let db = Arc::clone(&db);
        thread::spawn(move || handle_client(&mut stream.unwrap(), db));
    }

    Ok(())
}

fn main() -> io::Result<()> {
    use rsdb::expr;
    println!("{:?}", expr!(("dsadsa" (false) 2)));
    
    let db = Database::new();
    
    listen(Arc::new(db))?;

    Ok(())
}
