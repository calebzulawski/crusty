use crate::error::Result;
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

pub fn identifier<S: Into<String>>(name: S) -> Result<Expression> {
    Ok(Expression::Identifier(Identifier::new(name.into())?))
}

pub fn literal(value: Literal) -> Expression {
    Expression::Literal(value)
}

pub fn sizeof_type(the_type: Type) -> Expression {
    Expression::Sizeof(the_type)
}

pub fn ternary(condition: Expression, if_true: Expression, if_false: Expression) -> Expression {
    Expression::Ternary {
        condition: Box::new(condition),
        if_true: Box::new(if_true),
        if_false: Box::new(if_false),
    }
}

macro_rules! unary_gen {
    ($func_name:ident, $operation:ident) => {
        pub fn $func_name(expression: Expression) -> Expression {
            Expression::Unary {
                expression: Box::new(expression),
                operation: UnaryOperation::$operation,
            }
        }
    };
}

unary_gen!(prefix_inc, PrefixIncrement);
unary_gen!(prefix_dec, PrefixDecrement);
unary_gen!(postfix_inc, PostfixIncrement);
unary_gen!(postfix_dec, PostfixDecrement);
unary_gen!(address, Address);
unary_gen!(dereference, Dereference);
unary_gen!(plus, Plus);
unary_gen!(minus, Minus);
unary_gen!(bitwise_neg, BitwiseNegate);
unary_gen!(logical_neg, LogicalNegate);
unary_gen!(sizeof_exp, Sizeof);

pub fn cast(expression: Expression, to: Type) -> Expression {
    Expression::Unary {
        expression: Box::new(expression),
        operation: UnaryOperation::Cast(to),
    }
}

pub fn array_access(expression: Expression, index: Expression) -> Expression {
    Expression::Unary {
        expression: Box::new(expression),
        operation: UnaryOperation::ArrayAccess(Box::new(index)),
    }
}

pub fn struct_access<S: Into<String>>(expression: Expression, field: S) -> Result<Expression> {
    Ok(Expression::Unary {
        expression: Box::new(expression),
        operation: UnaryOperation::StructAccess(Identifier::new(field.into())?),
    })
}

pub fn struct_dereference<S: Into<String>>(expression: Expression, field: S) -> Result<Expression> {
    Ok(Expression::Unary {
        expression: Box::new(expression),
        operation: UnaryOperation::StructDereference(Identifier::new(field.into())?),
    })
}

macro_rules! binary_gen {
    ($func_name:ident, $operation:ident) => {
        pub fn $func_name(left: Expression, right: Expression) -> Expression {
            Expression::Binary {
                left: Box::new(left),
                right: Box::new(right),
                operation: BinaryOperation::$operation,
            }
        }
    };
}

binary_gen!(add, Add);
binary_gen!(subtract, Subtract);
binary_gen!(multiply, Multiply);
binary_gen!(divide, Divide);
binary_gen!(modulo, Modulo);
binary_gen!(left_shift, LeftShift);
binary_gen!(right_shift, RightShift);
binary_gen!(less_than, LessThan);
binary_gen!(greater_than, GreaterThan);
binary_gen!(less_than_equal, LessThanEqual);
binary_gen!(greater_than_equal, GreaterThanEqual);
binary_gen!(equal, Equal);
binary_gen!(not_equal, NotEqual);
binary_gen!(bitwise_and, BitwiseAnd);
binary_gen!(bitwise_or, BitwiseOr);
binary_gen!(bitwise_xor, BitwiseXor);
binary_gen!(logical_and, LogicalAnd);
binary_gen!(logical_or, LogicalOr);
binary_gen!(comma, Comma);
binary_gen!(assign, Assign);
binary_gen!(add_assign, AddAssign);
binary_gen!(subtract_assign, SubtractAssign);
binary_gen!(multiply_assign, MultiplyAssign);
binary_gen!(divide_assign, DivideAssign);
binary_gen!(modulo_assign, ModuloAssign);
binary_gen!(left_shift_assign, LeftShiftAssign);
binary_gen!(right_shift_assign, RightShiftAssign);
binary_gen!(bitwsie_and_assign, BitwiseAndAssign);
binary_gen!(bitwise_or_assign, BitwiseOrAssign);
binary_gen!(bitwise_xor_assign, BitwiseXorAssign);
