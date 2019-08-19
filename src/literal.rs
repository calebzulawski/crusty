#[derive(Debug)]
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

#[derive(Debug)]
pub struct FloatConstant {
    value: String,
}

impl FloatConstant {
    pub fn integer(integer: i128) -> Self {
        Self {
            value: integer.to_string(),
        }
    }

    pub fn decimal(integer: i128, fraction: i128) -> Self {
        Self {
            value: format!("{}.{}", integer, fraction),
        }
    }

    pub fn scientific(integer: i128, fraction: i128, exponent: i128) -> Self {
        Self {
            value: format!("{}.{}e{}", integer, fraction, exponent),
        }
    }

    pub fn from_float(float: f64) -> Self {
        Self {
            value: format!("{:.}", float),
        }
    }
}

impl std::fmt::Display for FloatConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}
