pub struct Ast {
    pub body: Vec<Expression>,
}

pub struct Block {
    pub body: Vec<Expression>,
}

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
}

pub enum Literal {
    Array,
    Tuple,
    Boolean(BooleanLiteral),
    Character(CharacterLiteral),
    String(StringLiteral),
    Number(NumberLiteral),
    Hexadecimal(HexadecimalLiteral),
    Binary(BinaryLiteral),
    Octal(OctalLiteral),
}

pub struct BooleanLiteral {
    pub value: bool,
}

pub struct CharacterLiteral {
    pub value: char,
}

pub struct StringLiteral {
    pub value: String,
}

pub struct NumberLiteral {
    pub value: f64,
}

pub struct HexadecimalLiteral {
    pub value: i64,
}

pub struct BinaryLiteral {
    pub value: i64,
}

pub struct OctalLiteral {
    pub value: i64,
}

pub struct Type {
    pub id: String,
}

pub struct Identifier {
    pub id: String,
}

pub struct Call {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

pub struct Loop {
    pub body: Block,
}

pub struct While {
    pub condition: Box<Expression>,
    pub body: Block,
}

pub struct IfChain {
    pub branches: Vec<IfBranch>,
}

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
pub struct Pattern {
    pub content: String,
}

pub struct Match {
    pub expression: Box<Expression>,
    pub branches: Vec<MatchArm>,
}

pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Block,
}

pub struct Member {
    pub path: Vec<Expression>,
}

pub struct VariableDeclaration {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub is_readonly: bool,
    pub initial_value: Option<Box<Expression>>,
}

pub struct FunctionParameter {
    pub name: Identifier,
    pub ty: Option<Type>,
}

pub struct FunctionParameters {
    pub items: Vec<FunctionParameter>,
}

pub struct GenericParameter {
    pub name: Identifier,
    pub bounds: Option<Vec<String>>,
}

pub struct FunctionGenericParameters {
    pub items: Vec<GenericParameter>,
}

pub struct FunctionBody {
    pub body: Block,
}

pub struct FunctionDeclaration {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub generic_params: Option<FunctionGenericParameters>,
    pub params: Option<FunctionParameters>,
    pub ret_ty: Option<Type>,
    pub body: Option<FunctionBody>,
}

pub enum Declaration {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
}
