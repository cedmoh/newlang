use crate::{
    eval::{Value, evaluate},
    runtime::GlobalScope,
};

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: Box<Expression>,
    pub arguments: CallArguments,
}

impl Call {
    pub fn get_callee_name(&self, global_scope: &mut GlobalScope) -> String {
        let callee = *self.callee.clone();

        if let Expression::Identifier(identifier) = callee {
            return identifier.id;
        }

        match evaluate(callee, global_scope) {
            Value::String(id) => id,
            Value::Pointer(id) => id,
            _ => panic!("Callee must be an identifier or evaluate to a string or pointer."),
        }
    }

    pub fn evaluate_arguments(&self, global_scope: &mut GlobalScope) -> Vec<Value> {
        self.arguments
            .items
            .iter()
            .map(|arg| evaluate(arg.clone(), global_scope))
            .collect()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CallArguments {
    pub items: Vec<Expression>,
}
