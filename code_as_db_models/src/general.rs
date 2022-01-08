use butane::db::{Connection, ConnectionSpec};

use crate::error::Result;

pub fn get_connection() -> Result<Connection> {
    Ok(butane::db::connect(&ConnectionSpec::load(
        ".butane/connection.json",
    )?)?)
}
