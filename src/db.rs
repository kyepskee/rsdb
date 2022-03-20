use std::collections::HashMap;

use sexplib::{parser, sexp::Atom, sexp::Expr};

use std::sync::Mutex;

pub struct Database {
    store: Mutex<HashMap<String, Expr>>,
}

impl Database {
    pub fn new() -> Self {
        let store = HashMap::new();
        Database {
            store: Mutex::new(store),
        }
    }

    pub fn get(&self, s: String) -> Expr {
        println!("getting {}", s);
        if let Some(x) = self.store.lock().unwrap().get(&s) {
            x.clone()
        } else {
            Expr::List(box vec![])
        }
    }

    pub fn set(&self, s: String, x: Expr) {
        println!("setting {} to {}", s, x.to_string());
        self.store.lock().unwrap().insert(s, x);
    }
}
