use crate::color_name_map;
use serenity::utils::Colour;
use hex;

pub fn parse_color(msg: &str) -> Option<Colour> {   
    if let Some(color) = parse_hex_color_from_msg(msg) {
        return Some(color);
    }
    if let Some(color) = parse_name_color_from_msg(msg) {
        return Some(color);
    }
    None
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
        Err(e) => { }
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