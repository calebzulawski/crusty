use crate::identifier::Identifier;
use crate::literal::Literal;
use crate::r#type::Type;

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Sizeof(Type),
    Unary {
        expression: Box<Expression>,
        operation: UnaryOperation,
    },
    Binary {
        left: Box<Expression>,
        right: Box<Expression>,
        operation: BinaryOperation,
    },
    Ternary {
        condition: Box<Expression>,
        if_true: Box<Expression>,
        if_false: Box<Expression>,
    },
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(val) => write!(f, "{}", val),
            Expression::Literal(val) => write!(f, "{}", val),
            Expression::Sizeof(val) => write!(f, "sizeof({})", val),
            Expression::Unary {
                expression,
                operation,
            } => match operation {
                UnaryOperation::PrefixIncrement => write!(f, "++({})", expression),
                UnaryOperation::PrefixDecrement => write!(f, "--({})", expression),
                UnaryOperation::PostfixIncrement => write!(f, "({})++", expression),
                UnaryOperation::PostfixDecrement => write!(f, "({})--", expression),
                UnaryOperation::Address => write!(f, "&({})", expression),
                UnaryOperation::Dereference => write!(f, "*({})", expression),
                UnaryOperation::Plus => write!(f, "+({})", expression),
                UnaryOperation::Minus => write!(f, "-({})", expression),
                UnaryOperation::BitwiseNegate => write!(f, "~({})", expression),
                UnaryOperation::LogicalNegate => write!(f, "!({})", expression),
                UnaryOperation::Sizeof => write!(f, "sizeof({})", expression),
                UnaryOperation::Cast(casted) => write!(f, "({})({})", casted, expression),
                UnaryOperation::ArrayAccess(index) => {
                    write!(f, "({})[{}]", expression, index.as_ref())
                }
                UnaryOperation::StructAccess(field) => write!(f, "({}).{}", expression, field),
                UnaryOperation::StructDereference(field) => {
                    write!(f, "({})->{}", expression, field)
                }
            },
            Expression::Binary {
                left,
                right,
                operation,
            } => {
                let symbol = match operation {
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
                write!(f, "{} {} {}", left, symbol, right)
            }
            Expression::Ternary {
                condition,
                if_true,
                if_false,
            } => write!(f, "({}) ? ({}) : ({})", condition, if_true, if_false),
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperation {
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
    ArrayAccess(Box<Expression>),
    StructAccess(Identifier),
    StructDereference(Identifier),
}

#[derive(Debug)]
pub enum BinaryOperation {
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
