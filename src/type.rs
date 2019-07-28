use crate::expression::Expression;
use crate::identifier::Identifier;

struct CVQualifier {
    constant: bool,
    volatile: bool,
}

impl std::fmt::Display for CVQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.constant {
            f.write_str("const")?;
        };
        if self.constant && self.volatile {
            f.write_str(" ")?;
        }
        if self.volatile {
            f.write_str("volatile")?;
        };
        Ok(())
    }
}

enum UnqualifiedType {
    Struct(Identifier),
    Union(Identifier),
    Alias(Identifier),
    Pointer(Box<QualifiedType>),
    Array(Box<ArrayType>),
}

struct ArrayType {
    unqualified_type: UnqualifiedType,
    size: Option<Expression>,
}

pub struct QualifiedType {
    qualifier: CVQualifier,
    unqualified_type: UnqualifiedType,
}

impl std::fmt::Display for QualifiedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
