use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use butane::{db::Connection, model, DataObject, ForeignKey};

#[model]
#[derive(Debug)]
#[rustfmt::skip]
pub struct Module {
    #[auto] id: i32,
    name: String,
    pub parent: Option<ForeignKey<Module>>, // TODO: Check for module cycles
    #[unique] hash: String,
    state: butane::ObjectState,
}

impl Module {
    pub fn new<S>(name: S, parent: Option<ForeignKey<Module>>) -> Self
    where
        S: Into<String>,
    {
        Self {
            id: -1,
            name: name.into(),
            parent,
            hash: Default::default(),
            state: Default::default(),
        }
    }

    pub fn save(&mut self, conn: &Connection) -> Result<(), butane::Error> {
        self.update_hash(conn);
        DataObject::save(self, conn)
    }

    // Update the unique hash of the function hash
    fn update_hash(&mut self, conn: &Connection) {
        let mut hasher = DefaultHasher::new();
        self.name.hash(&mut hasher);
        if let Some(ref parent_mod) = self.parent {
            parent_mod.load(conn).unwrap().pk().hash(&mut hasher);
        }

        self.hash = hasher.finish().to_string();
    }

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
