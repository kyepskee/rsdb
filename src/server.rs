use crate::db::Database;
use crate::io::read_expr;

use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};

use std::sync::Arc;
use std::thread;

use sexplib::parser::pp;
use sexplib::sexp::{Atom, Expr};

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
                .atom()
                .expect("Expected first argument to be atom")
                .str()
                .expect("Expected fisrt argument to be string");
            let res = pp::pp_atom(&db.get(addr));
            s.write(res.as_bytes()).unwrap();
        }
        if x[0] == Expr::Atom(Atom::Sym(String::from("get"))) {
            let addr = x[1]
                .atom()
                .expect("Expected first argument to be atom")
                .str()
                .expect("Expected fisrt argument to be string");

            let val = x[2].atom().expect("Expected second argument to be atom");
            db.set(addr, val);
        }
    } else {
    }
}

pub fn listen(db: Arc<Database>) -> io::Result<()> {
    use std::net::SocketAddr;
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 6969)))?;

    for stream in listener.incoming() {
        let db = Arc::clone(&db);
        thread::spawn(move || handle_client(&mut stream.unwrap(), db));
    }

    Ok(())
}
