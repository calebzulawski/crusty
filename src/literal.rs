pub enum Literal {
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

pub struct FloatConstant {
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
