mod math;
use clap::Parser;
use math::hex_to_rgb::convert_hex_to_rgb;
use math::rgb_to_hex::convert_rgb_to_hex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    // HEX TO RGB
    let my_hex = "0A543F";
    let rgb_tuple = convert_hex_to_rgb(my_hex);
    println!("HEX TO RGB: {:?}", rgb_tuple);

    // RGB TO HEX
    let my_rgb = "rgb(6, 58, 119)";
    let converted_hex_value = convert_rgb_to_hex(my_rgb);
    println!("RGB TO HEX: {}", converted_hex_value);

    println!("Hello {:?}!", cli.name)



    let leo = 'leo';





}
