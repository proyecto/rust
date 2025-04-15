use once_cell::sync::OnceCell;
use rusqlite::{Connection, Result};
use std::sync::Mutex;

static DB_INSTANCE: OnceCell<Mutex<Connection>> = OnceCell::new();

pub fn init(path: &str) -> Result<()> {
    let conn = Connection::open(path)?;
    DB_INSTANCE.set(Mutex::new(conn)).ok(); // ignoramos si ya está seteado
    Ok(())
}

pub fn get_connection() -> &'static Mutex<Connection> {
    DB_INSTANCE.get().expect("Database not initialized")
}