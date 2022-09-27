fn main() {
    // Comparing with a similar looking but different char
    // X: Latin X, 0x58
    // χ: Greek Chi, 0xCE 0xA7
    if 'X' == 'χ' {
        println!("It matches!");
    } else {
        println!("It doesn't match.");
    }
}