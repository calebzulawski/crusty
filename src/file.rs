use crate::declaration::Declaration;
use crate::identifier::Identifier;

pub enum IncludeMethod {
    Quote,
    Bracket,
}

pub struct Include {
    method: IncludeMethod,
    path: String,
}

impl std::fmt::Display for Include {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (quote1, quote2) = match &self.method {
            IncludeMethod::Quote => ("\"", "\""),
            IncludeMethod::Bracket => ("<", ">"),
        };
        write!(
            f,
            "#include {quote1}{path}{quote2}",
            path = self.path,
            quote1 = quote1,
            quote2 = quote2
        )
    }
}

pub struct Header {
    guard: Option<Identifier>,
    includes: Vec<Include>,
    declarations: Vec<Declaration>,
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(guard) = self.guard {
            writeln!(f, "#ifndef {}", guard)?;
            writeln!(f, "#define {}", guard)?;
        }
        for include in self.includes {
            writeln!(f, "{}", include)?;
        }
        for declaration in self.declarations {
            writeln!(f, "{}", declaration)?;
        }
        if let Some(guard) = self.guard {
            writeln!(f, "#endif // {}", guard)?;
        }
        Ok(())
    }
}

pub struct TranslationUnit {
    includes: Vec<Include>,
    declarations: Vec<Declaration>,
}

impl std::fmt::Display for TranslationUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for include in self.includes {
            writeln!(f, "{}", include)?;
        }
        for declaration in self.declarations {
            writeln!(f, "{}", declaration)?;
        }
        Ok(())
    }
}
