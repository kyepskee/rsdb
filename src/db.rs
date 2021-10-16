use std::collections::HashMap;
use crate::sexp::Atom;

use std::sync::Mutex;

pub struct Database {
    store: Mutex<HashMap<String, Atom>>
}

impl Database {
    pub fn new() -> Self {
        let store = HashMap::new();
        Database {
            store: Mutex::new(store)
        }
    }
    
    pub fn get(&self, s: String) -> Atom {
        println!("getting {}", s);
        self.store.lock().unwrap()[&s].clone()
    }
    
    pub fn set(&self, s: String, x: Atom) {
        println!("setting {} to {}", s, crate::parser::pp::pp_atom(&x));
        self.store.lock().unwrap().insert(s, x);
    }
}
