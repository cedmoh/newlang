#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Array,
    Tuple,
    Boolean(BooleanLiteral),
    Character(CharacterLiteral),
    String(StringLiteral),
    Decimal(DecimalLiteral),
    Hexadecimal(HexadecimalLiteral),
    Binary(BinaryLiteral),
    Octal(OctalLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CharacterLiteral {
    pub value: char,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DecimalLiteral {
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HexadecimalLiteral {
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryLiteral {
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OctalLiteral {
    pub value: i64,
}
