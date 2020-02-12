use serenity::utils::Colour;

pub fn parse_color(msg: &str) -> Colour {
    let color_arg = parse_color_arg(msg);
    match parse_color_arg(msg) {
        Some(color_arg) => {
            
        },
        None => eprintln!("Not enough arguments!");
    }
    Colour::DARK_RED
}

fn parse_color_arg(msg: &str) -> Option<&str> {
    let mut chunks = msg.split_whitespace();
    // We know it's at least size 1 since this was invoked via the color command
    chunks.next();
    return chunks.next();
}