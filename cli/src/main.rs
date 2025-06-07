use core::runner::run;

fn main() {
    let input = "
        myFunction fn {
            myVariable val 'hello' 
            ret myVariable
        }
        
        myFunction
    ";

    match run(input) {
        Ok(Some(val)) => println!("{}", val),
        Ok(None) => println!(""),
        Err(err) => eprintln!("{:?}", err),
    }
}
