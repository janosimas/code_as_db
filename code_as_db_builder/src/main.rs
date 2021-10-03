use butane::db::{Connection, ConnectionSpec};
use butane::prelude::*;
use code_as_db_models::function::Function;

fn get_connection() -> Connection {
    butane::db::connect(&ConnectionSpec::load(".butane/connection.json").unwrap()).unwrap()
}

fn main() {
    let conn = get_connection();
    let funcs = Function::query().load(&conn).unwrap();
    for func in funcs {
        println!("{}", func);
    }
}
