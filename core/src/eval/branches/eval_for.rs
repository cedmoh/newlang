use crate::{
    ast::For,
    eval::{GlobalScope, Value, eval_result::EvalResult, evaluate},
};

pub fn eval_for(global_scope: &mut GlobalScope, r#for: For) -> EvalResult {
    let mut last = EvalResult::finished(Value::Nil);

    // TODO: Handle return and break in iterator expression
    let iterator = evaluate(*r#for.iterator, global_scope);

    let Value::Map(map) = &iterator.value else {
        // Handle error: iterator is not iterable
        panic!("The provided iterator is not iterable.");
    };

    for (
        _key, // TODO: Implement destructuring pattern
        value,
    ) in map.iter()
    {
        global_scope.insert_value(r#for.item.id.clone(), value.clone());

        let EvalResult { value, flow } = evaluate(*r#for.body.clone(), global_scope);

        match flow {
            crate::eval::flow::Flow::Finished => last = EvalResult::finished(value),
            crate::eval::flow::Flow::Returned => return EvalResult::returned(value),
            crate::eval::flow::Flow::Broke => return EvalResult::broke(value),
            crate::eval::flow::Flow::Continued => continue,
        }
    }

    last
}
