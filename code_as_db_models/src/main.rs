#![feature(iter_intersperse)]

mod error;
mod function;
mod general;
mod module;
mod struct_m;

use crate::function::Function;
use crate::function::Parameter;
use crate::module::Module;

use butane::DataObject;

fn main() {
    let conn = general::get_connection().unwrap();
    // let mut f = Function::new_pub_free("hello", &[], "println!(\"Hello World!\")", None);
    let mut module = Module::new("my_mod1", None);
    module.save(&conn).unwrap();
    let mut module = Module::new("my_mod2", Some(module.into()));
    module.save(&conn).unwrap();

    let mut param1 = Parameter::new("name", "&str");
    param1.save(&conn).unwrap();

    let mut f = Function::new_pub_in_mod(
        "print_name3",
        &[param1],
        "println!(\"My name is {}\", name);",
        None,
        module,
    )
    .unwrap();
    f.save(&conn).unwrap();
}
