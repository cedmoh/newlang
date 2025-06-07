use newlang::{
    eval::{Functions, Variables, evaluate},
    parser::parse_program,
};

fn main() {
    let ast = parse_program(
        "myFunction fn {
            myVariable val '5'
            ret myVariable
        }
        
        myFunction",
    );

    let mut vars = Variables::default();
    let mut fns = Functions::default();

    let eval = ast
        .body
        .into_iter()
        .map(|ast| evaluate(ast, &mut vars, &mut fns))
        .collect::<Vec<_>>();

    println!("Result: {:#?}", eval);
    println!("Functions: {:#?}", fns);
}
