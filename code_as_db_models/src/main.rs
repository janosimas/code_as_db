mod function;
mod general;
mod module;
mod struct_m;

use crate::module::Module;
use crate::function::Function;
use crate::function::Parameter;

use butane::prelude::*;

fn main() {
    let conn = general::get_connection();
    // let mut f = Function::new_pub_free("hello", &[], "println!(\"Hello World!\")", None);
    let mut module = Module::new("my_mod1", None);
    module.save(&conn).unwrap();
    let mut module = Module::new("my_mod2", Some(module.into()));
    module.save(&conn).unwrap();

    let mut f = Function::new_pub_in_mod(
        "print_name3",
        &[Parameter::new("name", "&str")],
        "println!(\"My name is {}\", name);",
        None,
        module,
    );
    f.save(&conn).unwrap();
}
