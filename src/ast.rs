#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub body: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub body: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Block(Block),
    Declaration(Declaration),
    Loop(Loop),
    While(While),
    IfChain(IfChain),
    Match(Match),
    Member(Member),
    Call(Call),
    Identifier(Identifier),
    Literal(Literal),
    Dyadic(Dyadic),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DyadicOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dyadic {
    pub operator: DyadicOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub id: String,
}

impl<T: Into<String>> From<T> for Type {
    fn from(id: T) -> Self {
        Type { id: id.into() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub id: String,
}

impl<T: Into<String>> From<T> for Identifier {
    fn from(id: T) -> Self {
        Identifier { id: id.into() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: Identifier,
    pub arguments: CallArguments,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArguments {
    pub items: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub condition: Box<Expression>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfChain {
    pub branches: Vec<IfBranch>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfBranch {
    If {
        condition: Box<Expression>,
        body: Block,
    },
    ElseIf {
        condition: Box<Expression>,
        body: Block,
    },
    Else {
        body: Block,
    },
}

// TODO: Implement
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    pub expression: Box<Expression>,
    pub branches: Vec<MatchArm>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub path: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub is_readonly: bool,
    pub initial_value: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    pub name: Identifier,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FunctionParameters {
    pub items: Vec<FunctionParameter>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParameter {
    pub name: Identifier,
    pub bounds: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FunctionGenericParameters {
    pub items: Vec<GenericParameter>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub generic_params: FunctionGenericParameters,
    pub params: FunctionParameters,
    pub ret_ty: Option<Type>,
    pub body: Option<FunctionBody>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
}
