// 2 different behaviors seen
// debug mode:  `cargo run` will go to 127, and then panic on wrap to 128
// release mode:  `cargo run --release` will loop indefintely, wrapping from 127 to -128 to -1

fn main() {
    let mut counter : i8 = 0;
    loop {
        println!("{}", counter);
        counter += 1;
    }
}

