#![feature(box_syntax)]
#![feature(box_patterns)]

mod db;
mod io;
mod server;
use db::Database;

use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let db = Database::new();

    server::listen(Arc::new(db))?;

    Ok(())
}
