use super::shared::get_hex_from_digit;

fn remove_parenthesis(str: &str) -> &str {
    let text_check = str.find("(");

    if let Some(_) = text_check {
        let vals1: Vec<&str> = str.split("(").collect();
        let final_vals: Vec<&str> = vals1[1].split(")").collect();
        println!("{:?}", final_vals);
        final_vals[0]
    } else {
        str
    }
}

fn remove_delimiter(str: &str) -> Vec<&str> {
    if let Some(_) = str.find(", ") {
        str.split(", ").map(|f| -> &str { f.trim() }).collect()
    } else if let Some(_) = str.find(",") {
        str.split(",").collect()
    } else if let Some(_) = str.find(" ") {
        str.split(" ").collect()
    } else if let Some(_) = str.find(".") {
        str.split(".").collect()
    } else {
        panic!("Currently not supporting that formatting!")
    }
}

fn convert_rgb_part_to_hex_channel(val: u8) -> String {
    let quotient = val / 16;
    let remainder = val - (quotient * 16);

    let quotient_str = get_hex_from_digit(quotient).unwrap();
    let remainder_str = get_hex_from_digit(remainder).unwrap();

    quotient_str.to_owned() + remainder_str
}

fn rgb_to_hex(tuple: (u8, u8, u8)) -> String {
    format!(
        "#{}{}{}",
        convert_rgb_part_to_hex_channel(tuple.0),
        convert_rgb_part_to_hex_channel(tuple.1),
        convert_rgb_part_to_hex_channel(tuple.2)
    )
}

fn str_to_tuple(str: &str) -> (u8, u8, u8) {
    let str_without_parenthesis = remove_parenthesis(str);
    let vec = remove_delimiter(str_without_parenthesis);

    let rgb_tuple: (u8, u8, u8) = (
        vec[0].parse().unwrap(),
        vec[1].parse().unwrap(),
        vec[2].parse().unwrap(),
    );
    rgb_tuple
}

pub fn convert_rgb_to_hex(rgb_str: &str) -> String {
    let rgb_tuple = str_to_tuple(rgb_str);

    let hex_value = rgb_to_hex(rgb_tuple);

    hex_value
}
