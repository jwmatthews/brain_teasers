#![warn(clippy::pedantic)]

fn main() {
    let x = 0.1 + 0.2;
    if 0.1 + 0.2 == 0.3 {
        println!("Arithmetic still works");
    } else {
        println!("Please reboot the universe, x is '{}'", x);
    }
}