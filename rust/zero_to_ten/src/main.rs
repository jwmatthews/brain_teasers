use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
struct ZeroToTen(i32);

impl TryFrom<i32> for ZeroToTen {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 || value > 10 {
            Err("Value must be between 0 and 10")
        } else {
            Ok(Self(value))
        }
    }
}

fn main() {
    // TryFrom
    assert_eq!(ZeroToTen::try_from(8), Ok(ZeroToTen(8)));
    assert_eq!(ZeroToTen::try_from(-1), Err("Value must be between 0 and 10"));

    // TryInto
    let z: Result<ZeroToTen,&'static str>  = 10i32.try_into();
    assert_eq!(z, Ok(ZeroToTen(10)));

    let r: Result<ZeroToTen,&'static str>  = 11i32.try_into();
    assert_eq!(r, Err("Value must be between 0 and 10"));

}
