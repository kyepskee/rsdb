use crate::sexp::{Atom, Expr};

pub fn pp_atom(a: &Atom) -> String {
    match a {
        Atom::Num(n) => { n.to_string() },
        Atom::Str(s) => {
            "\"".to_string() + &s.escape_default().to_string() + "\""
        }
        Atom::Bool(b) => { b.to_string() }
        Atom::Sym(s) => { s.to_string() }
    }
}

pub fn pp_expr(e: &Expr) -> String {
    match e {
        Expr::Atom(a) => { pp_atom(a) }
        Expr::List(box v) => {
            let mut s = String::from("(");
            if v.len() > 0 {
                s.push_str(&pp_expr(&v[0]));
            }
            for ee in &v[1..] {
                s.push(' ');
                s.push_str(&pp_expr(ee))
            }
            s
        }
    }
}
