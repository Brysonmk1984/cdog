mod math;
use clap::Parser;
use math::hex_to_rgb::convert_hex_to_rgb;
use math::rgb_to_hex::convert_rgb_to_hex;

// TODO:
// 4. Clean up all the unwrap()
// 5. Make hex work with just 3 values

enum InputTypes {
    HEX,
    RGB,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'x', long)]
    /// Optional flag indicating a hexidecimal color is being entered
    hex: bool,
    #[arg(short, long)]
    /// Optional flag indicating an RGB value is being entered
    rgb: bool,
    /// Hex or RGB value to operate on
    value: Option<String>,
}

fn main() {
    let cli = Args::parse();

    if let Some(value) = cli.value.as_deref() {
        // Determine Color mode based on argument flags
        let mode = if cli.rgb {
            Some(InputTypes::RGB)
        } else if cli.hex {
            Some(InputTypes::HEX)
        } else {
            // if no flag was provided, try to determine color mode by the text input
            let mode_evaluation = determine_format(value);
            if let Some(InputTypes::HEX) = mode_evaluation {
                println!("Color Mode evaluated to be HEX")
            } else if let Some(InputTypes::RGB) = mode_evaluation {
                println!("Color Mode evaluated to be RGB")
            }
            mode_evaluation
        };

        // Based on the color mode, do appropriate color conversion
        let result = match mode {
            Some(InputTypes::RGB) => convert_rgb_to_hex(value),
            Some(InputTypes::HEX) => convert_hex_to_rgb(value.to_string()),
            None => panic!("Unable to determine color mode!"),
        };

        println!("INPUT: {} -> RESULT: {}", value, result);
    } else {
        println!("--> Please enter an RGB or HEX color value");
    }
}

// Used to try and determine color mode by the string input passed in
fn determine_format(input: &str) -> Option<InputTypes> {
    let result: Option<InputTypes>;
    if input.chars().nth(0).unwrap() == '#' || input.chars().all(char::is_alphanumeric) {
        result = Some(InputTypes::HEX);
    } else if let Some(_) = input.find(",") {
        result = Some(InputTypes::RGB);
    } else if let Some(_) = input.find(".") {
        result = Some(InputTypes::RGB);
    } else {
        result = None;
    }

    result
}
