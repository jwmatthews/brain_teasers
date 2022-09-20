fn main() {
    // if 'f32' then not enough resolution and 3.4028236 prints as 3.4028237
    // change to 'f64' and prints as expected
    const THREE_AND_A_BIT : f32 = 3.4028236;
    println!("{}", THREE_AND_A_BIT);
}
