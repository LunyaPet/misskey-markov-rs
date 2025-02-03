use std::fs::FileType;

use filetime::FileTime;
use reqwest::header::TE;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::conf::{read_instance, read_posting_token};

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

fn read_existing_file() -> Option<Vec<Post>> {
    let path = "./posts.json";

    let metadata = match std::fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return None,
    };

    if !metadata.is_file() {
        println!("File is not a file, deleting");
        std::fs::remove_file(path).unwrap();
        return None;
    }

    let modified = FileTime::from_last_modification_time(&metadata);
    let now = FileTime::now();

    if now.seconds() - modified.seconds() > 24 * 3600 * 7 {
        println!("File is older than a week, deleting");
        std::fs::remove_file(path).unwrap();
        return None;
    }

    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let posts: Vec<Post> = serde_json::from_reader(reader).unwrap();

    Some(posts)
}

fn write_file(posts: &Vec<Post>) {
    let path = "./posts.json";
    let file = std::fs::File::create(path).unwrap();
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &posts).unwrap();
}

fn get_posts_until_last(user_id: String, token: String, last_id: String) -> Vec<Post> {
    println!("{} Getting posts until last id: {}", user_id, last_id);
    let instance = read_instance();

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
            let mut posts = get_posts_until_last(user_id, token, last_id);

            out.append(&mut posts);
        }
        None => {
            println!("No posts found");
        }
    }

    out
}

pub fn get_posts(user_id: String, token: String) -> Vec<Post> {
    println!("Getting posts for {}", user_id);
    if let Some(posts) = read_existing_file() {
        return posts;
    }

    let mut posts = Vec::new();
    let instance = read_instance();

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
    let mut posts_2 = get_posts_until_last(user_id.clone(), token, last_id);
    posts.append(&mut posts_2);

    write_file(&posts);

    println!("Fetched {} posts from {}", &posts.len(), user_id);

    posts
}
