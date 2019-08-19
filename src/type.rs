use crate::error::Result;
use crate::expression::Expression;
use crate::identifier::Identifier;

#[derive(Debug)]
enum StructType {
    Struct,
    Union,
}

#[derive(Debug)]
struct Field {
    r#type: Box<Type>,
    name: Option<Identifier>,
    width: Option<Box<Expression>>,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.r#type.render(self.name.as_ref());
        if let Some(width) = &self.width {
            write!(f, "{} : {};", s, width)
        } else {
            write!(f, "{};", s)
        }
    }
}

#[derive(Debug)]
struct Enumerator {
    name: Identifier,
    value: Option<Box<Expression>>,
}

impl std::fmt::Display for Enumerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = &self.value {
            write!(f, "{} = {}", self.name, value)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

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
    Struct {
        name: Option<Identifier>,
        struct_type: StructType,
        fields: Option<Vec<Field>>,
    },
    Enum {
        name: Option<Identifier>,
        enumerators: Option<Vec<Enumerator>>,
    },
    Alias(Identifier),
    Void,
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
            BaseType::Struct {
                name,
                struct_type,
                fields,
            } => {
                f.write_str(match struct_type {
                    StructType::Struct => "struct",
                    StructType::Union => "union",
                })?;
                if let Some(name) = name {
                    write!(f, " {}", name)?;
                }
                if let Some(fields) = fields {
                    f.write_str(" {");
                    for field in fields {
                        write!(f, " {}", field)?;
                    }
                    f.write_str(" }");
                }
                Ok(())
            }
            BaseType::Enum { name, enumerators } => {
                f.write_str("enum")?;
                if let Some(name) = name {
                    write!(f, " {}", name)?;
                }
                if let Some(enumerators) = enumerators {
                    f.write_str(" {");
                    for (i, enumerator) in enumerators.iter().enumerate() {
                        write!(f, " {}", enumerator)?;
                        if i < enumerators.len() - 1 {
                            f.write_str(",");
                        }
                    }
                    f.write_str(" }");
                }
                Ok(())
            }
            BaseType::Alias(identifier) => write!(f, "{}", identifier),
            BaseType::Void => f.write_str("void"),
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
    pub(crate) fn render(&self, name: Option<&Identifier>) -> String {
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
        f.write_str(&self.render(None))
    }
}

macro_rules! terminate {
    ($func_name:ident, $base_type:ident) => {
        pub fn $func_name(self: Self) -> Type {
            let (qualifiers, modifiers) = self.get_qualifiers_modifiers();
            Type {
                base: BaseType::$base_type,
                qualifiers: qualifiers,
                modifiers: modifiers,
            }
        }
    };
}

macro_rules! implement_type_builder {
    () => (
    terminate!(void, Void);
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

    pub fn alias_named<S: Into<String>>(self, name: S) -> Result<Type> {
        let (qualifiers, modifiers) = self.get_qualifiers_modifiers();
        Ok(Type {
            base: BaseType::Alias(Identifier::new(name.into())?),
            qualifiers: qualifiers,
            modifiers: modifiers,
        })
    }

    pub fn struct_named<S: Into<String>>(self, name: S) -> Result<StructBuilder> {
        let (qualifiers, modifiers) = self.get_qualifiers_modifiers();
        Ok(StructBuilder {
            qualifiers: qualifiers,
            modifiers: modifiers,
            name: Identifier::new(name.into())?,
            struct_type: StructType::Struct,
        })
    }

    pub fn union_named<S: Into<String>>(self, name: S) -> Result<StructBuilder> {
        let (qualifiers, modifiers) = self.get_qualifiers_modifiers();
        Ok(StructBuilder {
            qualifiers: qualifiers,
            modifiers: modifiers,
            name: Identifier::new(name.into())?,
            struct_type: StructType::Union,
        })
    }

    pub fn anonymous_struct(self) -> StructDefinitionBuilder {
        let (qualifiers, modifiers) = self.get_qualifiers_modifiers();
        StructDefinitionBuilder {
            qualifiers: qualifiers,
            modifiers: modifiers,
            name: None,
            struct_type: StructType::Struct,
            fields: Vec::new(),
        }
    }

    pub fn anonymous_union(self) -> StructDefinitionBuilder {
        let (qualifiers, modifiers) = self.get_qualifiers_modifiers();
        StructDefinitionBuilder {
            qualifiers: qualifiers,
            modifiers: modifiers,
            name: None,
            struct_type: StructType::Union,
            fields: Vec::new(),
        }
    }

    pub fn enum_named<S: Into<String>>(self, name: S) -> Result<EnumBuilder> {
        let (qualifiers, modifiers) = self.get_qualifiers_modifiers();
        Ok(EnumBuilder {
            qualifiers: qualifiers,
            modifiers: modifiers,
            name: Identifier::new(name.into())?,
        })
    }

    pub fn anonymous_enum(self) -> EnumDefinitionBuilder {
        let (qualifiers, modifiers) = self.get_qualifiers_modifiers();
        EnumDefinitionBuilder {
            qualifiers: qualifiers,
            modifiers: modifiers,
            name: None,
            enumerators: Vec::new(),
        }
    }
    )
}

pub struct TypeBuilder {
    modifiers: Vec<TypeModifier>,
}

impl TypeBuilder {
    fn get_qualifiers_modifiers(self) -> (Qualifiers, Vec<TypeModifier>) {
        (Qualifiers::none(), self.modifiers)
    }
    implement_type_builder!();

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
    fn get_qualifiers_modifiers(self) -> (Qualifiers, Vec<TypeModifier>) {
        (self.qualifiers, self.modifiers)
    }
    implement_type_builder!();

    pub fn pointer_to(self) -> TypeBuilder {
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

pub struct StructBuilder {
    qualifiers: Qualifiers,
    modifiers: Vec<TypeModifier>,
    name: Identifier,
    struct_type: StructType,
}

impl StructBuilder {
    fn finish(self) -> Type {
        Type {
            base: BaseType::Struct {
                name: Some(self.name),
                struct_type: self.struct_type,
                fields: None,
            },
            qualifiers: self.qualifiers,
            modifiers: self.modifiers,
        }
    }

    fn with_fields(self) -> StructDefinitionBuilder {
        StructDefinitionBuilder {
            qualifiers: self.qualifiers,
            modifiers: self.modifiers,
            name: Some(self.name),
            struct_type: self.struct_type,
            fields: Vec::new(),
        }
    }
}

pub struct StructDefinitionBuilder {
    qualifiers: Qualifiers,
    modifiers: Vec<TypeModifier>,
    name: Option<Identifier>,
    struct_type: StructType,
    fields: Vec<Field>,
}

impl StructDefinitionBuilder {
    pub fn finish(self) -> Type {
        Type {
            base: BaseType::Struct {
                name: self.name,
                struct_type: self.struct_type,
                fields: Some(self.fields),
            },
            qualifiers: self.qualifiers,
            modifiers: self.modifiers,
        }
    }

    pub fn named_field<S: Into<String>>(mut self, r#type: Type, name: S) -> Result<Self> {
        self.fields.push(Field {
            r#type: Box::new(r#type),
            name: Some(Identifier::new(name.into())?),
            width: None,
        });
        Ok(self)
    }

    pub fn anonymous_field(mut self, r#type: Type) -> Self {
        self.fields.push(Field {
            r#type: Box::new(r#type),
            name: None,
            width: None,
        });
        self
    }

    pub fn named_bit_field<S: Into<String>>(
        mut self,
        r#type: Type,
        name: S,
        width: Expression,
    ) -> Result<Self> {
        self.fields.push(Field {
            r#type: Box::new(r#type),
            name: Some(Identifier::new(name.into())?),
            width: Some(Box::new(width)),
        });
        Ok(self)
    }

    pub fn anonymous_bit_field(mut self, r#type: Type, width: Expression) -> Self {
        self.fields.push(Field {
            r#type: Box::new(r#type),
            name: None,
            width: Some(Box::new(width)),
        });
        self
    }
}

pub struct EnumBuilder {
    qualifiers: Qualifiers,
    modifiers: Vec<TypeModifier>,
    name: Identifier,
}

impl EnumBuilder {
    pub fn finish(self) -> Type {
        Type {
            base: BaseType::Enum {
                name: Some(self.name),
                enumerators: None,
            },
            qualifiers: self.qualifiers,
            modifiers: self.modifiers,
        }
    }

    pub fn with_enumerators(self) -> EnumDefinitionBuilder {
        EnumDefinitionBuilder {
            qualifiers: self.qualifiers,
            modifiers: self.modifiers,
            name: Some(self.name),
            enumerators: Vec::new(),
        }
    }
}

pub struct EnumDefinitionBuilder {
    qualifiers: Qualifiers,
    modifiers: Vec<TypeModifier>,
    name: Option<Identifier>,
    enumerators: Vec<Enumerator>,
}

impl EnumDefinitionBuilder {
    pub fn finish(self) -> Type {
        Type {
            base: BaseType::Enum {
                name: self.name,
                enumerators: Some(self.enumerators),
            },
            qualifiers: self.qualifiers,
            modifiers: self.modifiers,
        }
    }

    pub fn enumerator<S: Into<String>>(mut self, name: S) -> Result<Self> {
        self.enumerators.push(Enumerator {
            name: Identifier::new(name.into())?,
            value: None,
        });
        Ok(self)
    }

    pub fn enumerator_with_value<S: Into<String>>(
        mut self,
        name: S,
        value: Expression,
    ) -> Result<Self> {
        self.enumerators.push(Enumerator {
            name: Identifier::new(name.into())?,
            value: Some(Box::new(value)),
        });
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unnamed_type() {
        let t = TypeBuilder::new()
            .sized_array_of(Expression::Literal(crate::Literal::Signed(5)))
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
            .unwrap()
            .finish();
        assert_eq!(
            format!("{}", t),
            "const union u * volatile (* * volatile [5])(int, __m256i * const)"
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
            t.render(Some(&Identifier::new("foo").unwrap())),
            "long double (* const * foo)()"
        );
    }

    #[test]
    fn empty_anon_enum() {
        let t = TypeBuilder::new().anonymous_enum().finish();
        assert_eq!(
            t.render(Some(&Identifier::new("foo").unwrap())),
            "enum { } foo"
        );
    }

    #[test]
    fn empty_anon_struct() {
        let t = TypeBuilder::new().anonymous_struct().finish();
        assert_eq!(
            t.render(Some(&Identifier::new("foo").unwrap())),
            "struct { } foo"
        );
    }

    #[test]
    fn empty_anon_union() {
        let t = TypeBuilder::new().anonymous_union().finish();
        assert_eq!(
            t.render(Some(&Identifier::new("foo").unwrap())),
            "union { } foo"
        );
    }

    #[test]
    fn empty_named_enum() {
        let t = TypeBuilder::new()
            .enum_named("foo")
            .unwrap()
            .with_enumerators()
            .finish();
        assert_eq!(
            t.render(Some(&Identifier::new("bar").unwrap())),
            "enum foo { } bar"
        );
    }

    #[test]
    fn empty_named_struct() {
        let t = TypeBuilder::new()
            .struct_named("foo")
            .unwrap()
            .with_fields()
            .finish();
        assert_eq!(
            t.render(Some(&Identifier::new("bar").unwrap())),
            "struct foo { } bar"
        );
    }

    #[test]
    fn empty_named_union() {
        let t = TypeBuilder::new()
            .union_named("foo")
            .unwrap()
            .with_fields()
            .finish();
        assert_eq!(
            t.render(Some(&Identifier::new("bar").unwrap())),
            "union foo { } bar"
        );
    }

    #[test]
    fn struct_fields() {
        let t = TypeBuilder::new()
            .anonymous_struct()
            .anonymous_field(TypeBuilder::new().int())
            .named_field(TypeBuilder::new().array_of().pointer_to().void(), "foo")
            .unwrap()
            .named_bit_field(
                TypeBuilder::new().char(),
                "bar",
                Expression::Literal(crate::Literal::Signed(1)),
            )
            .unwrap()
            .finish();
        assert_eq!(
            format!("{}", t),
            "struct { int; void * foo[]; char bar : 1; }"
        );
    }

    #[test]
    fn enumerators() {
        let t = TypeBuilder::new()
            .anonymous_enum()
            .enumerator("FOO")
            .unwrap()
            .enumerator_with_value("BAR", Expression::Literal(crate::Literal::Signed(1)))
            .unwrap()
            .finish();
        assert_eq!(format!("{}", t), "enum { FOO, BAR = 1 }");
    }
}
