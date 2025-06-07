use newlang::{
    evaluator::{Functions, Variables, evaluate},
    parser::parse_program,
};

fn main() {
    let ast = parse_program("2+2");
    let eval = evaluate(
        ast.body.into_iter().next().unwrap(),
        &mut Variables::default(),
        &mut Functions::default(),
    );

    println!("Result: {:?}", eval)
}
