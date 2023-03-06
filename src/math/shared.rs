use strum_macros::EnumString;

#[allow(dead_code)]
#[derive(EnumString, Debug, PartialEq)]

//enum with explicit discriminator
enum HexAlpha {
    A = 10,
    B = 11,
    C = 12,
    D = 13,
    E = 14,
    F = 15,
}

// Return numerical value from hex digit
pub fn get_hex_digit_value(val: &str) -> Option<u8> {
    match val.to_lowercase().as_str() {
        "a" => Some(HexAlpha::A as u8),
        "b" => Some(HexAlpha::B as u8),
        "c" => Some(HexAlpha::C as u8),
        "d" => Some(HexAlpha::D as u8),
        "e" => Some(HexAlpha::E as u8),
        "f" => Some(HexAlpha::F as u8),
        "0" => Some(0),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        _ => None,
    }
}

pub fn get_hex_from_digit(val: u8) -> Option<&'static str> {
    match val {
        0 => Some("0"),
        1 => Some("1"),
        2 => Some("2"),
        3 => Some("3"),
        4 => Some("4"),
        5 => Some("5"),
        6 => Some("6"),
        7 => Some("7"),
        8 => Some("8"),
        9 => Some("9"),
        10 => Some("A"),
        11 => Some("B"),
        12 => Some("C"),
        13 => Some("D"),
        14 => Some("E"),
        15 => Some("F"),
        _ => None,
    }
}
