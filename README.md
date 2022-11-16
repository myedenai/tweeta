# Tweeta
A Simple Rust lib by MY EDEN AI for creating Twitter bots

And Tweet 😄
```rust
use tweeta::*;

fn main() {
  let example_bot = TwitterBot::new()
    .consumer_key(YOUR_CONSUMER_KEY)
    .consumer_secret_key(YOUR_CONSUMER_SECRET_KEY)
    .access_token(YOUR_ACCESS_TOKEN)
    .secret_access_token(YOUR_SECRET_ACCESS_TOKEN);

  let res = example_bot.tweet("🐦 + 🦀 = 💙 #myfirstTweet", None).unwrap();

  println!("{:?}", res);
}
```
<br/>

**Congratulations ! 🎉** 