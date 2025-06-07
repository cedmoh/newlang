use super::super::*;

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
