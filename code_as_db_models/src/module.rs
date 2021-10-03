use butane::{ForeignKey, model};

#[model]
#[derive(Debug, Default)]
#[rustfmt::skip]
pub struct Module {
    #[auto] id: i32,
    name: String,
    // pub parent: Option<ForeignKey<Module>>, // TODO: Check for module cycles
    #[unique] hash: String,
}

impl Module {
    /// Get a reference to the module's id.
    pub fn id(&self) -> &i32 {
        &self.id
    }

    /// Get a reference to the module's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Set the module's name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
