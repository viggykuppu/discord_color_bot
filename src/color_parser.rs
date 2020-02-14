use crate::color_name_map;
use serenity::utils::Colour;
use std::error;
use hex;
use std::fmt;

pub fn parse_color(msg: &str) -> Result<Colour, ColorParseError> {   
    if let Some(color) = parse_hex_color_from_msg(msg) {
        if is_valid_grey(&color){
            return Ok(color);
        } else {
            return Err(ColorParseError::InvalidGrey);
        }
    } else if let Some(color) = parse_name_color_from_msg(msg) {
        if is_valid_grey(&color) {
            return Ok(color);
        } else {
            return Err(ColorParseError::InvalidGrey);
        }
    }
    Err(ColorParseError::InvalidColor)
}

// Certain shades of grey cause user's names to blend in with the background and be invisible in Discord
// This checks for colors in that range and prevents the user from setting their name color to it
// The exact color of discord's bg is #36393f
// We want to block all shades of grey from #1a1a1a - #737373
fn is_valid_grey(color: &Colour) -> bool {
    let (r,g,b) = color.tuple();
    let r_i32 = r as i32; 
    let g_i32 = g as i32;
    let b_i32 = b as i32;

    // Checks for "greyish" colors
    let min_diff = 32;
    if (r_i32 - g_i32).abs() <= min_diff && (r_i32 - b_i32).abs() <= min_diff && (g_i32 - b_i32).abs() <= min_diff {
        let lower_bound = r >= 26 && g >= 26 && b >= 26;
        let upper_bound = r <= 115 && g <= 115 && b <= 115;

        return !(upper_bound && lower_bound);
    }
    return true;
}

fn parse_hex_color_from_msg(msg: &str) -> Option<Colour> {
    let mut chunks = msg.split_whitespace();
    chunks.next();
    if let Some(color_arg) = chunks.next() {
        return parse_hex_color(color_arg);
    }
    None
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
        Err(_) => { }
    }
    None
}

fn parse_name_color_from_msg(msg: &str) -> Option<Colour> {
    let mut chunks = msg.split_whitespace();
    chunks.next();
    let color_arg = chunks.fold(String::new(), |mut acc, s| {
        acc.push_str(&s.to_lowercase());
        return acc;
    });
    return parse_name_color(&color_arg)
}

fn parse_name_color(color_arg: &str) -> Option<Colour> {
    match color_name_map::COLOR_NAME_MAP.get::<str>(color_arg) {
        Some(color_hex) => return parse_hex_color(color_hex),
        None => return None
    }
}

#[derive(Debug)]
pub enum ColorParseError {
    InvalidColor,
    InvalidGrey
}

impl fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColorParseError::InvalidColor => write!(f, "Invalid argument"),
            ColorParseError::InvalidGrey => write!(f, "Invalid shade of grey")
        }
    }
}

impl error::Error for ColorParseError { }