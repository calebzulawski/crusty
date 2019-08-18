use crate::error::{Error, Result};

#[derive(Debug)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new<S: Into<String>>(name: S) -> Result<Self> {
        let name = name.into();
        if name.is_empty() {
            return Err(Error::BadIdentifier(name));
        }
        let first = name.chars().next().unwrap();
        if !first.is_ascii_alphabetic() && (first != '_') {
            return Err(Error::BadIdentifier(name));
        }
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || (c == '_'))
        {
            return Err(Error::BadIdentifier(name));
        }
        Ok(Self { name: name })
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let ident = Identifier::new("_Some_valid_ident5").unwrap();
        assert_eq!(format!("{}", ident), "_Some_valid_ident5");
    }

    #[test]
    fn num_start() {
        Identifier::new("1test").unwrap_err();
    }

    #[test]
    fn bad_char() {
        Identifier::new("hello/world").unwrap_err();
    }
}
