pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new(name: &str) -> Result<Self, String> {
        if name.is_empty() {
            return Err("identifier cannot be empty".to_string());
        }
        let first = name.chars().next().unwrap();
        if !first.is_ascii_alphabetic() && (first != '_') {
            return Err("identifier must begin with letter or '_'".to_string());
        }
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || (c == '_'))
        {
            return Err("identifiers can only contain alphanumerics or '_'".to_string());
        }
        Ok(Self {
            name: name.to_string(),
        })
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
