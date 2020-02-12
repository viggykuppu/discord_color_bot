use std::env;


fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = discord_name_color::init(&token);

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}