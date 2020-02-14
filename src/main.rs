use std::process;


fn main() {
    if let Err(why) = discord_name_color::run() {
        eprintln!("An error occurred while starting the client: {:?}", why);
        process::exit(1)
    }
}