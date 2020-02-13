use serenity::utils::Colour;
use hex;

pub fn parse_color(msg: &str) -> Colour {   
    match parse_color_arg(msg) {
        Some(color_arg) => {
            if let Some(color) = parse_hex_color(color_arg) {
                return color;
            }
        },
        None => eprintln!("Not enough arguments!")
    }
    Colour::DARK_RED
}

fn parse_color_arg(msg: &str) -> Option<&str> {
    let mut chunks = msg.split_whitespace();
    // We know it's at least size 1 since this was invoked via the color command
    chunks.next();
    return chunks.next();
}

fn parse_hex_color(color_arg: &str) -> Option<Colour> {
    let mut hex_arg = color_arg;
    if color_arg.starts_with("#") {
        hex_arg = &color_arg[1..];
    }
    match hex::decode(hex_arg) {
        Ok(hex) => {
            if hex.len() == 3 {
                return Some(Colour::from_rgb(hex[0], hex[1], hex[2]));
            }
        },
        Err(e) => { eprintln!("Error parsing supplied hex color: {}",e) }
    }
    None
}

