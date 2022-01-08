use core::fmt;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use butane::{db::Connection, model, DataObject, ForeignKey, Many};

use crate::{general, module::Module, struct_m::Struct, error::Result, error::Error};

#[model]
#[derive(Debug)]
#[rustfmt::skip]
pub struct Parameter {
    #[auto] id: i32,
    name: String,
    type_name: String,
    state: butane::ObjectState,
}

impl Parameter {
    pub fn new<S>(name: S, type_name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            id: -1,
            name: name.into(),
            type_name: type_name.into(),
            state: Default::default(),
        }
    }

    /// Get a reference to the parameter's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get a reference to the parameter's type name.
    pub fn type_name(&self) -> &str {
        self.type_name.as_str()
    }
}

#[model]
#[derive(Debug)]
#[rustfmt::skip]
pub struct Function {
    #[auto] id: i32,
    is_public: bool,
    name: String,
    parameters: Many<Parameter>,
    return_type: Option<String>,
    body: String,
    parent_mod: Option<ForeignKey<Module>>,
    parent_strunc: Option<ForeignKey<Struct>>,
    #[unique] hash: String,
    state: butane::ObjectState,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let return_type = self
            .return_type
            .as_ref()
            .map_or(String::default(), |return_type| {
                format!(" -> {}", return_type)
            });

        let conn = general::get_connection().unwrap();
        let parameters = self.parameters.load(&conn).unwrap();
        let parameters: String = parameters
            .into_iter()
            .map(|param| format!("{}: {}", param.name, param.type_name))
            .intersperse(", ".to_string())
            .collect();

        write!(
            f,
            "{access}fn {name}({parameters}){return_type} {{ {body} }}",
            access = if self.is_public { "pub " } else { "" },
            name = self.name(),
            body = self.body(),
            return_type = return_type,
            parameters = parameters,
        )
    }
}

impl Function {
    pub fn new_pub_free<S>(
        name: S,
        parameters: &[Parameter],
        body: S,
        return_type: Option<S>,
    ) -> Result<Self>
    where
        S: Into<String>,
    {
        let mut params = Many::<Parameter>::default();
        for par in parameters {
            if par.id.is_negative() {
                return Err(Error::InvalidParameter)
            }

            params.add(par);
        }

        Ok(Self {
            id: -1,
            is_public: true,
            name: name.into(),
            parameters: params,
            return_type: return_type.map(Into::into),
            body: body.into(),
            parent_mod: None,
            parent_strunc: None,
            hash: Default::default(),
            state: Default::default(),
        })
    }

    pub fn new_pub_in_mod<S>(
        name: S,
        parameters: &[Parameter],
        body: S,
        return_type: Option<S>,
        parent_mod: Module,
    ) -> Result<Self>
    where
        S: Into<String>,
    {
        let mut func = Self::new_pub_free(name, parameters, body, return_type)?;
        func.parent_mod = Some(parent_mod.into());
        Ok(func)
    }

    pub fn save(&mut self, conn: &Connection) -> Result<()> {
        self.update_hash(conn)?;

        // let list = self.parameters.load(conn)?;
        // for param in  {
        //     param.save(conn)?
        // }

        // self.parameters.save(conn)?;
        Ok(DataObject::save(self, conn)?)
    }

    // Update the unique hash of the function hash
    fn update_hash(&mut self, conn: &Connection) -> Result<()> {
        let mut hasher = DefaultHasher::new();
        self.name.hash(&mut hasher);
        if let Some(ref parent_mod) = self.parent_mod {
            parent_mod.load(conn)?.pk().hash(&mut hasher);
        }
        if let Some(ref parent_strunc) = self.parent_strunc {
            parent_strunc.load(conn)?.pk().hash(&mut hasher);
        }
        self.hash = hasher.finish().to_string();
        Ok(())
    }

    pub fn pk(&self) -> &i32 {
        &self.id
    }

    /// Get a reference to the function's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Set the function's name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get a reference to the function's return type.
    pub fn return_type(&self) -> Option<&String> {
        self.return_type.as_ref()
    }

    /// Set the function's return type.
    pub fn set_return_type(&mut self, return_type: Option<String>) {
        self.return_type = return_type;
    }

    /// Get a reference to the function's body.
    pub fn body(&self) -> &str {
        self.body.as_str()
    }

    /// Set the function's body.
    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }

    /// Get a reference to the function's parameters.
    pub fn parameters(&self) -> &Many<Parameter> {
        &self.parameters
    }

    /// Set the function's parameters.
    pub fn set_parameters(&mut self, parameters: &Parameter) {
        self.parameters.add(parameters);
    }
}
