use crate::error::Result;
use crate::expression::Expression;
use crate::identifier::Identifier;

#[derive(Debug)]
struct Qualifiers {
    constant: bool,
    volatile: bool,
}

impl Qualifiers {
    fn none() -> Self {
        Self {
            constant: false,
            volatile: false,
        }
    }

    fn is_none(&self) -> bool {
        !self.constant && !self.volatile
    }

    fn set_const(&mut self, value: bool) {
        self.constant = value;
    }
    fn set_volatile(&mut self, value: bool) {
        self.volatile = value;
    }
}

impl std::fmt::Display for Qualifiers {
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

#[derive(Debug)]
enum BaseType {
    Struct(Identifier),
    Union(Identifier),
    Enum(Identifier),
    Alias(Identifier),
    Char,
    SignedChar,
    UnsignedChar,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Float,
    Double,
    LongDouble,
}

impl std::fmt::Display for BaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseType::Struct(identifier) => write!(f, "struct {}", identifier),
            BaseType::Union(identifier) => write!(f, "union {}", identifier),
            BaseType::Enum(identifier) => write!(f, "enum {}", identifier),
            BaseType::Alias(identifier) => write!(f, "{}", identifier),
            BaseType::Char => f.write_str("char"),
            BaseType::SignedChar => f.write_str("signed char"),
            BaseType::UnsignedChar => f.write_str("unsigned char"),
            BaseType::Short => f.write_str("short"),
            BaseType::UnsignedShort => f.write_str("unsigned short"),
            BaseType::Int => f.write_str("int"),
            BaseType::UnsignedInt => f.write_str("unsigned int"),
            BaseType::Long => f.write_str("long"),
            BaseType::UnsignedLong => f.write_str("unsigned long"),
            BaseType::LongLong => f.write_str("long long"),
            BaseType::UnsignedLongLong => f.write_str("unsigned long long"),
            BaseType::Float => f.write_str("float"),
            BaseType::Double => f.write_str("double"),
            BaseType::LongDouble => f.write_str("long double"),
        }
    }
}

#[derive(Debug)]
enum TypeModifier {
    Pointer(Qualifiers),
    Array(Option<Box<Expression>>),
    Function(Vec<Type>),
}

#[derive(Debug)]
pub struct Type {
    base: BaseType,
    qualifiers: Qualifiers,
    modifiers: Vec<TypeModifier>,
}

impl Type {
    pub(crate) fn to_string(&self, name: Option<&Identifier>) -> String {
        let mut v = if let Some(name) = name {
            vec![format!("{}", name)]
        } else {
            Vec::new()
        };

        // Keep track of which side of the identifier we're on, for the spiral rule
        let mut right = true;
        for modifier in &self.modifiers {
            match modifier {
                TypeModifier::Pointer(qualifiers) => {
                    // Pointers don't need parens, so just move to the left
                    right = false;

                    // Print the cv-qualified pointer
                    if qualifiers.is_none() {
                        v.insert(0, "*".to_string());
                    } else {
                        v.insert(0, format!("* {}", qualifiers));
                    }
                    if v.len() > 1 {
                        v.insert(1, " ".to_string());
                    }
                }
                TypeModifier::Function(args) => {
                    // Push us back to the right if we're on the left
                    if !right && !v.is_empty() {
                        v.insert(0, "(".to_string());
                        v.push(")".to_string());
                    }
                    right = true;

                    // Print the function arguments
                    v.push("(".to_string());
                    v.extend(args.iter().enumerate().map(|(i, x)| {
                        if i > 0 {
                            format!(", {}", x)
                        } else {
                            format!("{}", x)
                        }
                    }));
                    v.push(")".to_string());
                }
                TypeModifier::Array(size) => {
                    // Push us back to the right if we're on the left
                    if !right && !v.is_empty() {
                        v.insert(0, "(".to_string());
                        v.push(")".to_string());
                    }
                    right = true;

                    // Print the array and size
                    v.push("[".to_string());
                    if let Some(size) = size {
                        v.push(format!("{}", size))
                    }
                    v.push("]".to_string());
                }
            }
        }
        v.insert(
            0,
            if self.qualifiers.is_none() {
                format!("{}", self.base)
            } else {
                format!("{} {}", self.qualifiers, self.base)
            },
        );
        if v.len() > 1 {
            v.insert(1, " ".to_string());
        }
        v.join("")
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string(None))
    }
}

macro_rules! terminate {
    ($func_name:ident, $base_type:ident) => {
        pub fn $func_name(self) -> Type {
            let (qualifiers, modifiers) = self.get_qualified_modifiers();
            Type {
                base: BaseType::$base_type,
                qualifiers: qualifiers,
                modifiers: modifiers,
            }
        }
    };
}

macro_rules! terminate_ident {
    ($func_name:ident, $base_type:ident) => {
        pub fn $func_name<S: Into<String>>(self, name: S) -> Result<Type> {
            let (qualifiers, modifiers) = self.get_qualified_modifiers();
            Ok(Type {
                base: BaseType::$base_type(Identifier::new(name.into())?),
                qualifiers: qualifiers,
                modifiers: modifiers,
            })
        }
    };
}

macro_rules! functions {
    () => {
        terminate_ident!(struct_named, Struct);
        terminate_ident!(union_named, Union);
        terminate_ident!(enum_named, Enum);
        terminate_ident!(alias_named, Alias);
        terminate!(char, Char);
        terminate!(unsigned_char, UnsignedChar);
        terminate!(signed_char, SignedChar);
        terminate!(short, Short);
        terminate!(unsigned_short, UnsignedShort);
        terminate!(int, Int);
        terminate!(unsigned_int, UnsignedInt);
        terminate!(long, Long);
        terminate!(unsigned_long, UnsignedLong);
        terminate!(long_long, LongLong);
        terminate!(unsigned_long_long, UnsignedLongLong);
        terminate!(float, Float);
        terminate!(double, Double);
        terminate!(long_double, LongDouble);
    };
}

pub struct TypeBuilder {
    modifiers: Vec<TypeModifier>,
}

impl TypeBuilder {
    fn get_qualified_modifiers(self) -> (Qualifiers, Vec<TypeModifier>) {
        (Qualifiers::none(), self.modifiers)
    }
    functions!();

    pub fn new() -> Self {
        Self {
            modifiers: Vec::new(),
        }
    }

    pub fn pointer_to(mut self) -> Self {
        self.modifiers
            .push(TypeModifier::Pointer(Qualifiers::none()));
        self
    }

    fn function_returning(mut self, args: Vec<Type>) -> Self {
        self.modifiers.push(TypeModifier::Function(args));
        self
    }

    pub fn array_of(mut self) -> Self {
        self.modifiers.push(TypeModifier::Array(None));
        self
    }

    pub fn sized_array_of(mut self, size: Expression) -> Self {
        self.modifiers
            .push(TypeModifier::Array(Some(Box::new(size))));
        self
    }

    pub fn constant(self) -> QualifiedTypeBuilder {
        QualifiedTypeBuilder {
            modifiers: self.modifiers,
            qualifiers: Qualifiers {
                constant: true,
                volatile: false,
            },
        }
    }

    pub fn volatile(self) -> QualifiedTypeBuilder {
        QualifiedTypeBuilder {
            modifiers: self.modifiers,
            qualifiers: Qualifiers {
                constant: false,
                volatile: true,
            },
        }
    }

    pub fn constant_volatile(self) -> QualifiedTypeBuilder {
        QualifiedTypeBuilder {
            modifiers: self.modifiers,
            qualifiers: Qualifiers {
                constant: true,
                volatile: true,
            },
        }
    }
}

pub struct QualifiedTypeBuilder {
    modifiers: Vec<TypeModifier>,
    qualifiers: Qualifiers,
}

impl QualifiedTypeBuilder {
    fn get_qualified_modifiers(self) -> (Qualifiers, Vec<TypeModifier>) {
        (self.qualifiers, self.modifiers)
    }
    functions!();

    pub fn pointer_to(mut self) -> TypeBuilder {
        let QualifiedTypeBuilder {
            mut modifiers,
            qualifiers,
        } = self;
        modifiers.push(TypeModifier::Pointer(qualifiers));
        TypeBuilder {
            modifiers: modifiers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unnamed_type() {
        let t = TypeBuilder::new()
            .volatile()
            .pointer_to()
            .pointer_to()
            .function_returning(vec![
                TypeBuilder::new().int(),
                TypeBuilder::new()
                    .constant()
                    .pointer_to()
                    .alias_named("__m256i")
                    .unwrap(),
            ])
            .volatile()
            .pointer_to()
            .constant()
            .union_named("u")
            .unwrap();
        assert_eq!(
            format!("{}", t),
            "const union u * volatile (* * volatile)(int, __m256i * const)"
        );
    }

    #[test]
    fn named_type() {
        let t = TypeBuilder::new()
            .pointer_to()
            .constant()
            .pointer_to()
            .function_returning(Vec::new())
            .long_double();
        assert_eq!(
            t.to_string(Some(&Identifier::new("foo").unwrap())),
            "long double (* const * foo)()"
        );
    }
}
