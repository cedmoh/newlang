pub mod ast;
pub mod eval;
pub mod parser;
pub mod runtime;

#[cfg(test)]
mod tests {
    use crate::runtime::Runtime;

    #[test]
    fn it_works() {
        let program = r#"
            my [
              internal: fn {
                'hello'
              }
            ] 
            
            print my.internal
        "#;

        Runtime::new().run(program);
    }
}
