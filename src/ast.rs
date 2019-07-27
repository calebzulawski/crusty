struct Body {
    includes: Vec<Include>,
    declarations: Vec<Declaration>,
}

enum IncludeMethod {
    Quote,
    Bracket,
}

struct Include {
    method: IncludeMethod,
    path: String,
}

enum Literal {
    Signed(i128),
    SignedLong(i128),
    SignedLongLong(i128),
    Unsigned(u128),
    UnsignedLong(u128),
    UnsignedLongLong(u128),
    Character(char),
    Float(FloatConstant),
    Double(FloatConstant),
    LongDouble(FloatConstant),
    String(String),
}

struct FloatConstant {
    integer: i128,
    fraction: u128,
    exponent: i128,
}

enum Expression {
    Identifier(String),
    Literal(Literal),
    Sizeof(QualifiedType),
    Unary(Box<UnaryExpression>),
    Binary(Box<BinaryExpression>),
    Ternary(Box<TernaryExpression>),
    Assignment(Box<AssignmentExpression>),
}

struct UnaryExpression {
    expression: Expression,
    operation: UnaryOperation,
}

enum UnaryOperation {
    PrefixIncrement,
    PrefixDecrement,
    PostfixIncrement,
    PostfixDecrement,
    Address,
    Dereference,
    Plus,
    Minus,
    Negate,
    LogicalNegate,
    Sizeof,
    Cast(UnqualifiedType),
    ArrayAccess(Expression),
    StructAccess(String),
    StructDereference(String),
}

struct BinaryExpression {
    left: Expression,
    right: Expression,
    operation: BinaryOperation,
}

enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    LeftShift,
    RightShift,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LogicalAnd,
    LogicalOr,
    Comma,
}

struct TernaryExpression {
    condition: Expression,
    if_true: Expression,
    if_false: Expression,
}

struct AssignmentExpression {
    left: Expression,
    right: Expression,
    operation: AssignmentOperation,
}

enum AssignmentOperation {
    Assign,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    LeftShift,
    RightShift,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
}

enum Storage {
    Auto,
    Static,
    Register,
    Extern,
    Typedef,
}

struct Qualifier {
    constant: bool,
    volatile: bool,
}

enum UnqualifiedType {
    Struct(String),
    Union(String),
    Identifier(String),
    Pointer(Box<QualifiedType>),
    Array(Box<ArrayType>),
}

struct ArrayType {
    unqualified_type: UnqualifiedType,
    size: Option<Expression>,
}

struct QualifiedType {
    qualifier: Qualifier,
    unqualified_type: UnqualifiedType,
}

struct Declaration {
    storage: Option<Storage>,
    qualifier: Qualifier,
    qualified_type: QualifiedType,
}

enum StructType {
    Struct,
    Union,
}

struct Member {
    qualified_type: QualifiedType,
    name: String,
    bitfield: Expression,
}

struct StructDeclaration {
    name: Option<String>,
    members: Option<Vec<Member>>,
    struct_type: StructType,
}

struct Enumerator {
    name: String,
    value: Option<Expression>,
}

struct EnumDeclaration {
    name: String,
    enumerators: Vec<Enumerator>,
}
