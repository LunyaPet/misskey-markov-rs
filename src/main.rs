use std::process::exit;

use markov::Chain;

mod conf;
mod posts;

fn main() {
    let mut posts: Vec<posts::Post> = Vec::new();
    let mut chain = Chain::new();

    let accounts = conf::read_accounts();

    for account in accounts {
        let mut fetched_posts = posts::get_posts(account.id, account.token);
        posts.append(&mut fetched_posts);
    }

    for post in posts {
        if post.text.is_none() {
            continue;
        }

        chain.feed_str(post.text.unwrap().as_str());
    }

    let mut str = String::new();

    let chunk_1 = chain.generate_str();
    str.push_str(&chunk_1);
    let mut last_token = chunk_1.split_whitespace().last().unwrap().to_string();

    for _ in 1..conf::read_multiplier() {
        let chunk = chain.generate_str_from_token(&last_token);
        str.push_str(&chunk);
        last_token = chunk.split_whitespace().last().unwrap().to_string();
    }

    println!("{}", &str);

    posts::create_post(str);

    exit(0);
}
