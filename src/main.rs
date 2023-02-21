mod math;
use math::hex_to_rgb::convert_hex_to_rgb;
use math::rgb_to_hex::convert_rgb_to_hex;

fn main() {
    // HEX TO RGB
    let my_hex = "0A543F";
    let rgb_tuple = convert_hex_to_rgb(my_hex);
    println!("HEX TO RGB: {:?}", rgb_tuple);

    // RGB TO HEX
    let my_rgb = "rgb(6, 58, 119)";
    let converted_hex_value = convert_rgb_to_hex(my_rgb);
    println!("RGB TO HEX: {}", converted_hex_value);
}
