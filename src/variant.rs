use std::fmt::{self, Write};

use crate::formatter::Formatter;
use crate::{docs::Docs, fields::Fields};

use crate::r#type::Type;

/// Defines an enum variant.
#[derive(Debug, Clone)]
pub struct Variant {
    name: String,
    fields: Fields,
    docs: Option<Docs>,
    attributes: Vec<String>,
}

impl Variant {
    /// Return a new enum variant with the given name.
    pub fn new(name: &str) -> Self {
        Variant {
            name: name.to_string(),
            fields: Fields::Empty,
            docs: None,
            attributes: vec![],
        }
    }

    /// Add a comment line to the variant
    pub fn doc(&mut self, docs: &str) -> &mut Self {
        self.docs = Some(Docs::new(docs));
        self
    }

    /// Add an attribute for the variant
    pub fn attr(&mut self, attribute: &str) -> &mut Self {
        self.attributes.push(attribute.to_string());
        self
    }

    /// Add a named field to the variant.
    pub fn named<T>(&mut self, name: &str, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.fields.named(name, ty);
        self
    }

    /// Add a tuple field to the variant.
    pub fn tuple(&mut self, ty: &str) -> &mut Self {
        self.fields.tuple(ty);
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for attr in self.attributes.iter() {
            write!(fmt, "#[{}]\n", attr)?;
        }
        if let Some(ref docs) = self.docs {
            docs.fmt(fmt)?;
        }
        write!(fmt, "{}", self.name)?;
        self.fields.fmt(fmt)?;
        write!(fmt, ",\n")?;

        Ok(())
    }
}
