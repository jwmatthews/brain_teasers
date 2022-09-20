use std::io::stdin;

fn main() {
    println!("What is 3+2? Type your answer and press enter.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read standard input");

    if input == "5" {
        println!("Correct!");
    } else {
        println!("Incorrect!");
        println!("Response entered is `{:#?}`", input);
        println!("Need to use .trim() to strip the control characters: `{:#?}`", input.trim());
    }
}
