use std::fmt::{self, Write};

use crate::formatter::Formatter;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Docs {
    docs: String,
}

impl Docs {
    pub fn new(docs: &str) -> Self {
        Docs {
            docs: docs.to_string(),
        }
    }

    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for line in self.docs.lines() {
            write!(fmt, "/// {}\n", line)?;
        }

        Ok(())
    }
}
