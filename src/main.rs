use std::process::exit;

use markov::Chain;
use serde_json::json;

mod cred;
mod posts;

fn main() {
    let posts = posts::get_posts("a0cj5mqxoz2e0001".to_string());
    println!("Fetched posts: {}", posts.len());
    let mut chain = Chain::new();
    for post in posts {
        if post.text.is_none() {
            continue;
        }

        if post.cw.is_some() {
            continue;
        }

        chain.feed_str(post.text.as_ref().unwrap().as_str());
    }

    println!("{}", chain.generate_str());

    exit(0);
}
