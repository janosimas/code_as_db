mod function;
mod general;
mod module;
mod struct_m;

use crate::function::Function;
use crate::function::Parameter;

fn main() {
    let conn = general::get_connection();
    // let mut f = Function::new_pub_free("hello", &[], "println!(\"Hello World!\")", None);
    let mut f = Function::new_pub_free(
        "print_name",
        &[Parameter::new("name", "&str")],
        "println!(\"My name is {}\", name);",
        None,
    );
    f.save(&conn).unwrap();
}
