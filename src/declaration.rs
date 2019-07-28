use crate::expression::Expression;
use crate::identifier::Identifier;
use crate::r#type::QualifiedType;

pub enum Storage {
    Auto,
    Static,
    Register,
    Extern,
    Typedef,
}

impl std::fmt::Display for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Storage::Auto => "auto",
                Storage::Static => "static",
                Storage::Register => "register",
                Storage::Extern => "extern",
                Storage::Typedef => "typedef",
            }
        )
    }
}

pub struct Declaration {
    storage: Option<Storage>,
    qualified_type: QualifiedType,
}

enum StructType {
    Struct,
    Union,
}

struct Member {
    qualified_type: QualifiedType,
    name: Identifier,
    bitfield: Expression,
}

struct StructDeclaration {
    name: Option<Identifier>,
    members: Option<Vec<Member>>,
    struct_type: StructType,
}

struct Enumerator {
    name: Identifier,
    value: Option<Expression>,
}

struct EnumDeclaration {
    name: Identifier,
    enumerators: Vec<Enumerator>,
}
