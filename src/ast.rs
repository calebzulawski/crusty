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

enum Literal {
    Signed(i128),
    SignedLong(i128),
    SignedLongLong(i128),
    Unsigned(u128),
    UnsignedLong(u128),
    UnsignedLongLong(u128),
    Character(char),
    WideCharacter(char),
    Float(FloatConstant),
    Double(FloatConstant),
    LongDouble(FloatConstant),
    String(String),
    WideString(Vec<char>),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Signed(val) => write!(f, "{}", val),
            Literal::SignedLong(val) => write!(f, "{}L", val),
            Literal::SignedLongLong(val) => write!(f, "{}LL", val),
            Literal::Unsigned(val) => write!(f, "{}U", val),
            Literal::UnsignedLong(val) => write!(f, "{}UL", val),
            Literal::UnsignedLongLong(val) => write!(f, "{}ULL", val),
            Literal::Character(val) => write!(f, "'{}'", val.escape_default()),
            Literal::WideCharacter(val) => write!(f, "L'{}'", val.escape_default()),
            Literal::Float(val) => write!(f, "{}f", val),
            Literal::Double(val) => write!(f, "{}", val),
            Literal::LongDouble(val) => write!(f, "{}L", val),
            Literal::String(val) => write!(f, "\"{}\"", val.as_str().escape_default()),
            Literal::WideString(val) => write!(
                f,
                "L\"{}\"",
                val.iter()
                    .map(|x| x.escape_default().to_string())
                    .collect::<String>()
            ),
        }
    }
}

struct FloatConstant {
    integer: i128,
    fraction: u128,
    exponent: i128,
}

impl std::fmt::Display for FloatConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{integer}.{fraction}e{exponent}",
            integer = self.integer,
            fraction = self.fraction,
            exponent = self.exponent
        )
    }
}

enum Expression {
    Identifier(String),
    Literal(Literal),
    Sizeof(QualifiedType),
    Unary(Box<UnaryExpression>),
    Binary(Box<BinaryExpression>),
    Ternary(Box<TernaryExpression>),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(val) => write!(f, "{}", val),
            Expression::Literal(val) => write!(f, "{}", val),
            Expression::Sizeof(val) => write!(f, "sizeof({})", val),
            Expression::Unary(val) => write!(f, "{}", val),
            Expression::Binary(val) => write!(f, "{}", val),
            Expression::Ternary(val) => write!(f, "{}", val),
        }
    }
}

struct UnaryExpression {
    expression: Expression,
    operation: UnaryOperation,
}

impl std::fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.operation {
            UnaryOperation::PrefixIncrement => write!(f, "++({})", self.expression),
            UnaryOperation::PrefixDecrement => write!(f, "--({})", self.expression),
            UnaryOperation::PostfixIncrement => write!(f, "({})++", self.expression),
            UnaryOperation::PostfixDecrement => write!(f, "({})--", self.expression),
            UnaryOperation::Address => write!(f, "&({})", self.expression),
            UnaryOperation::Dereference => write!(f, "*({})", self.expression),
            UnaryOperation::Plus => write!(f, "+({})", self.expression),
            UnaryOperation::Minus => write!(f, "-({})", self.expression),
            UnaryOperation::BitwiseNegate => write!(f, "~({})", self.expression),
            UnaryOperation::LogicalNegate => write!(f, "!({})", self.expression),
            UnaryOperation::Sizeof => write!(f, "sizeof({})", self.expression),
            UnaryOperation::Cast(casted) => write!(
                f,
                "({casted})({exp})",
                casted = casted,
                exp = self.expression
            ),
            UnaryOperation::ArrayAccess(index) => {
                write!(f, "({exp})[{index}]", exp = self.expression, index = index)
            }
            UnaryOperation::StructAccess(field) => {
                write!(f, "({exp}).{field}", exp = self.expression, field = field)
            }
            UnaryOperation::StructDereference(field) => {
                write!(f, "({exp})->{field}", exp = self.expression, field = field)
            }
        }
    }
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
    BitwiseNegate,
    LogicalNegate,
    Sizeof,
    Cast(QualifiedType),
    ArrayAccess(Expression),
    StructAccess(String),
    StructDereference(String),
}

struct BinaryExpression {
    left: Expression,
    right: Expression,
    operation: BinaryOperation,
}

impl std::fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self.operation {
            BinaryOperation::Add => "+",
            BinaryOperation::Subtract => "-",
            BinaryOperation::Multiply => "*",
            BinaryOperation::Divide => "/",
            BinaryOperation::Modulo => "%",
            BinaryOperation::LeftShift => "<<",
            BinaryOperation::RightShift => ">>",
            BinaryOperation::LessThan => "<",
            BinaryOperation::GreaterThan => ">",
            BinaryOperation::LessThanEqual => "<=",
            BinaryOperation::GreaterThanEqual => ">=",
            BinaryOperation::Equal => "==",
            BinaryOperation::NotEqual => "!=",
            BinaryOperation::BitwiseAnd => "&",
            BinaryOperation::BitwiseOr => "|",
            BinaryOperation::BitwiseXor => "^",
            BinaryOperation::LogicalAnd => "&&",
            BinaryOperation::LogicalOr => "||",
            BinaryOperation::Comma => ",",
            BinaryOperation::Assign => "=",
            BinaryOperation::AddAssign => "+=",
            BinaryOperation::SubtractAssign => "-=",
            BinaryOperation::MultiplyAssign => "*=",
            BinaryOperation::DivideAssign => "/=",
            BinaryOperation::ModuloAssign => "%=",
            BinaryOperation::LeftShiftAssign => "<<=",
            BinaryOperation::RightShiftAssign => ">>=",
            BinaryOperation::BitwiseAndAssign => "&=",
            BinaryOperation::BitwiseOrAssign => "|=",
            BinaryOperation::BitwiseXorAssign => "^=",
        };
        write!(
            f,
            "{left} {symbol} {right}",
            left = self.left,
            symbol = symbol,
            right = self.right
        )
    }
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
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    LeftShiftAssign,
    RightShiftAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
}

struct TernaryExpression {
    condition: Expression,
    if_true: Expression,
    if_false: Expression,
}

impl std::fmt::Display for TernaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({condition}) ? ({if_true}) : ({if_false})",
            condition = self.condition,
            if_true = self.if_true,
            if_false = self.if_false
        )
    }
}

enum Storage {
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

struct CVQualifier {
    constant: bool,
    volatile: bool,
}

impl std::fmt::Display for CVQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = if self.constant { "const" } else { "" };
        let v = if self.volatile { "volatile" } else { "" };
        write!(f, "{c} {v}", c = c, v = v)
    }
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
    qualifier: CVQualifier,
    unqualified_type: UnqualifiedType,
}

impl std::fmt::Display for QualifiedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

struct Declaration {
    storage: Option<Storage>,
    qualifier: CVQualifier,
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
