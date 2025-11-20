use crate::eval::{Value, flow::Flow};

#[derive(Debug, Clone)]
pub struct EvalResult {
    pub value: Value,
    pub flow: Flow,
}

impl EvalResult {
    pub fn new(value: Value, flow: Flow) -> Self {
        Self { value, flow }
    }

    pub fn finished(value: Value) -> Self {
        Self {
            value,
            flow: Flow::Finished,
        }
    }

    pub fn returned(value: Value) -> Self {
        Self {
            value,
            flow: Flow::Returned,
        }
    }

    pub fn broke(value: Value) -> Self {
        Self {
            value,
            flow: Flow::Broke,
        }
    }

    pub fn continued(value: Value) -> Self {
        Self {
            value,
            flow: Flow::Continued,
        }
    }
}
