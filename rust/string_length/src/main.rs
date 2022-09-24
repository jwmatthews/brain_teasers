const HELLO_WORLD : &'static str = "Halló heimur";

fn main() {
    play();
    play2();
    play3();
    play4();
}

fn play() -> Option<char> {
    println!("{} is {} characters long.", HELLO_WORLD, HELLO_WORLD.len());
    let n = HELLO_WORLD.chars().nth(4)?;
    println!("the 4th char is '{}'", n);
    return Some(n)
}

fn play2() -> () {
    println!("{} is {} characters long.", HELLO_WORLD, HELLO_WORLD.len());
    let _n = match HELLO_WORLD.chars().nth(4) {
        Some(x) => println!("the 4th char is '{}'", x),
        None => println!("Unable to find 4th char")
    };
}

fn play3() -> () {
    println!("{} is {} characters long.", HELLO_WORLD, HELLO_WORLD.len());
    let n = HELLO_WORLD.chars().nth(4).unwrap();
    println!("the 4th char is '{}'", n);
}

fn play4() -> () {
    println!("{} is {} characters long.", HELLO_WORLD, HELLO_WORLD.len());
    let n = HELLO_WORLD.chars().nth(4).expect("4th char should have existed");
    println!("the 4th char is '{}'", n);
}
