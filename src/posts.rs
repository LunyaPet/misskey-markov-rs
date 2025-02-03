use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::cred::{read_instance, read_token};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub username: String,
    pub host: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub text: Option<String>,
    pub cw: Option<String>,
    pub user: User,
}

#[derive(Serialize, Deserialize)]
struct Posts {
    posts: Vec<Post>,
}

fn get_posts_until_last(user_id: String, last_id: String) -> Vec<Post> {
    println!("Getting posts until last id: {}", last_id);
    let instance = read_instance();
    let token = read_token();

    let mut out = reqwest::blocking::Client::new()
        .post(format!("https://{}/api/users/notes", instance))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "userId": user_id,
            "limit": 100,
            "untilId": last_id,
            "withRenotes": false,
            "withBots": false,
        }))
        .send()
        .unwrap()
        .json::<Vec<Post>>()
        .unwrap();

    match out.last() {
        Some(last_el) => {
            let last_id = last_el.id.clone();
            let mut posts = get_posts_until_last(user_id, last_id);

            out.append(&mut posts);
        }
        None => {
            println!("No posts found");
        }
    }

    out
}

pub fn get_posts(user_id: String) -> Vec<Post> {
    let mut posts = Vec::new();
    let instance = read_instance();
    let token = read_token();

    reqwest::blocking::Client::new()
        .post(format!("https://{}/api/users/notes", instance))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "userId": user_id,
            "limit": 100,
            "withRenotes": false,
            "withBots": false,
        }))
        .send()
        .unwrap()
        .json::<Vec<Post>>()
        .unwrap()
        .iter()
        .for_each(|post| {
            posts.push(Post {
                id: post.id.clone(),
                text: post.text.clone(),
                cw: post.cw.clone(),
                user: User {
                    name: post.user.name.clone(),
                    username: post.user.username.clone(),
                    host: post.user.host.clone(),
                },
            });
        });

    let last_id = posts.last().unwrap().id.clone();
    let mut posts_2 = get_posts_until_last(user_id, last_id);
    posts.append(&mut posts_2);

    posts
}
