use std::mem::size_of;

// If we don't have the below decoration
// the 3 byte structure will occupy 4 bytes due to 32bit memory addressing performance optimizations
#[repr(C, packed)]
struct VeryImportantMessage {
    _message_type: u8,
    _destination: u16
}

fn main() {
    println!("VeryImportantMessage occupies {} bytes.", size_of::<VeryImportantMessage>());
}

