use butane::model;

#[model]
#[derive(Debug, Default)]
pub struct Struct {
    #[auto]
    id: i32,
    name: String,
}

impl Struct {
    /// Get a reference to the struct's pk.
    pub fn pk(&self) -> &i32 {
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
