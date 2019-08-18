use crate::identifier::Identifier;
use crate::literal::Literal;
use crate::r#type::Type;

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Sizeof(Type),
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

#[derive(Debug)]
pub struct UnaryExpression {
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

#[derive(Debug)]
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
    Cast(Type),
    ArrayAccess(Expression),
    StructAccess(Identifier),
    StructDereference(Identifier),
}

#[derive(Debug)]
pub struct BinaryExpression {
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct TernaryExpression {
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
