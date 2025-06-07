use super::Block;

#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
    pub body: Block,
}
