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

    let str = chain.generate_str();

    println!("{}", &str);

    posts::create_post(str);

    exit(0);
}
