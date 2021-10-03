use butane::db::{Connection, ConnectionSpec};

pub fn get_connection() -> Connection {
    butane::db::connect(&ConnectionSpec::load(".butane/connection.json").unwrap()).unwrap()
}
