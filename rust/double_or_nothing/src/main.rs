//
// Rust does _not_ support function overloading.
//

/*
fn double_it(n: i32) -> i32 {
    n * 2
}

fn double_it(n: f32) -> f32 {
    n * 2.0
}

fn main() {
    println!("2 * 4 = {}", double_it(2));
}
// Above won't compile:
// error[E0428]: the name `double_it` is defined multiple times
//
*/

fn double_it<T>(n: T) -> T
where T: std::ops::Mul<Output = T> + From<i32> {
    n * 2.into()
}

fn main() {
    println!("2+2 = {}", double_it(2));
}
