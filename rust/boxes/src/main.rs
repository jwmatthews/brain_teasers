fn main() {
    // Will crash in debug mode as ~40mb array is allocated on stack and then moved to the Box
    // Will run as expected in release mode
    //let c = Box::new([0u32; 10_000_000]);
    //println!("{}", c.len());

    let x = vec![0u32; 10_000_000];
    println!("{}", x.len());
}
