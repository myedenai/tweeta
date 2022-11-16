use std::{collections::HashMap, path::Path};

use tweeta::*;



fn main() {
    let example_bot = Tweeta::new()
        .consumer_key(YOUR_CONSUMER_KEY)
        .consumer_secret_key(YOUR_CONSUMER_SECRET_KEY)
        .access_token(YOUR_ACCESS_TOKEN)
        .secret_access_token(YOUR_SECRET_ACCESS_TOKEN);

    let mut params = HashMap::new();
    let media_id: &str = &example_bot
        .upload_file(Path::new("assets/profile.jpg"))
        .unwrap()
        .to_string();

    params.insert("media_ids", media_id);
    let res = example_bot
        .tweet("This tweet has an image", Some(params))
        .unwrap();

    println!("{:?}", res);
}