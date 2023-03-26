use super::shared::get_hex_digit_value;

// Given the HEX channel parts, return the single RGB value part
fn convert_hex_channel_to_rgb(channel_quotient: u8, channel_remainder: u8) -> u8 {
    let val = channel_quotient * 16;
    let final_value = val + channel_remainder;

    final_value
}

// Handles conversion from hex to rgb
pub fn convert_hex_to_rgb(hex: String) -> String {
    let has_hash = hex.chars().nth(0).unwrap() == '#';
    let hex_len = if has_hash { hex.len() - 1 } else { hex.len() };

    println!("{}", hex_len);

    let hex_str = if has_hash {
        hex[1..].to_string()
    } else {
        hex[0..].to_string()
    };

    let hex_tuple = if hex_len == 6 {
        (
            String::from(&hex_str[0..2]),
            String::from(&hex_str[2..4]),
            String::from(&hex_str[4..]),
        )
    } else if hex_len == 3 {
        let first = format!("{}{}", &hex_str[0..1].to_owned(), &hex_str[0..1].to_owned());
        let second = format!("{}{}", &hex_str[1..2].to_owned(), &hex_str[1..2].to_owned());
        let third = format!("{}{}", &hex_str[2..3].to_owned(), &hex_str[2..3].to_owned());

        (first, second, third)
    } else {
        panic!("Incorrect Hex number count")
    };

    // r
    let x1 = get_hex_digit_value(&hex_tuple.0[..1]).unwrap();
    let y1 = get_hex_digit_value(&hex_tuple.0[1..]).unwrap();
    // g
    let x2 = get_hex_digit_value(&hex_tuple.1[..1]).unwrap();
    let y2 = get_hex_digit_value(&hex_tuple.1[1..]).unwrap();
    // b
    let x3 = get_hex_digit_value(&hex_tuple.2[..1]).unwrap();
    let y3 = get_hex_digit_value(&hex_tuple.2[1..]).unwrap();

    let r = convert_hex_channel_to_rgb(x1, y1);
    let b = convert_hex_channel_to_rgb(x2, y2);
    let g = convert_hex_channel_to_rgb(x3, y3);

    format!("{},{},{}", r, g, b)
}
