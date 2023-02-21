use super::shared::get_hex_digit_value;

// Given the HEX channel parts, return the single RGB value part
fn convert_hex_channel_to_rgb(channel_quotient: u8, channel_remainder: u8) -> u8 {
    let val = channel_quotient * 16;
    let final_value = val + channel_remainder;

    final_value
}

// Handles conversion from hex to rgb
pub fn convert_hex_to_rgb(hex: &str) -> String {
    let hex_tuple = if hex.chars().nth(0).unwrap() == '#' {
        let hex_string_without_hash = &hex[1..];
        (
            &hex_string_without_hash[0..2],
            &hex_string_without_hash[2..4],
            &hex_string_without_hash[4..],
        )
    } else {
        (&hex[0..2], &hex[2..4], &hex[4..])
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
