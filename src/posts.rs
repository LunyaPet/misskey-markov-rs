use filetime::FileTime;
use regex::Regex;
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

    if let Some(last_post) = posts.last() {
        let last_id = last_post.id.clone();
        let mut posts_2 = get_posts_until_last(user_id.clone(), token, last_id);
        posts.append(&mut posts_2);
    }

    write_file(&posts);

    println!("Fetched {} posts from {}", &posts.len(), user_id);

    posts
}

#[derive(Serialize, Deserialize)]
pub struct CreatedNote {
    created_note: Post,
}

pub fn create_post(text: String) {
    let instance = read_instance();
    let posting_token = read_posting_token();

    let res = reqwest::blocking::Client::new()
        .post(format!("https://{}/api/notes/create", instance))
        .header("Authorization", format!("Bearer {}", posting_token.trim()))
        .json(&json!({
            "text": text,
            "cw": "Markov Generated Post",
        }))
        .send()
        .unwrap()
        .json::<CreatedNote>()
        .unwrap();

    println!("https://{}/notes/{}", instance, res.created_note.id);
}

pub fn sanitize_mentions(text: String) -> String {
    // @<mention> and @<mention>@<instance> are not allowed
    // regex
    let re = Regex::new(r"(@\w+)(@[\w.]+)?").unwrap();
    re.replace_all(&text, "<plain>$1$2</plain>").to_string()
}

#[cfg(test)]
#[serial_test::serial]
mod tests {
    use super::*;

    #[test]
    fn test_get_posts() {
        let user_id = std::env::var("MISSKEY_USER_ID").unwrap();
        let token = std::env::var("MISSKEY_TOKEN").unwrap();

        let config = format!(
            r#"
posting_token: none
instance: social.mldchan.dev
accounts:
  - id: {user_id}
    token: {token}
"#,
            user_id = user_id,
            token = token
        );

        if std::fs::exists("posts.json").unwrap() {
            std::fs::remove_file("posts.json").unwrap();
        }

        std::fs::write("config.yml", config).unwrap();

        let posts = get_posts(user_id, token);
        assert!(posts.len() > 0);
    }

    // Test sanitize mentions method
    #[test]
    fn test_sanitize_mentions() {
        let mention = "@markov";
        let expected = "<plain>@markov</plain>";
        let result = sanitize_mentions(mention.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sanitize_mentions_instance() {
        let mention = "@markov@mldchan.dev";
        let expected = "<plain>@markov@mldchan.dev</plain>";
        let result = sanitize_mentions(mention.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sanitize_mentions_multiple() {
        let mention = "@markov@mldchan.dev @markov";
        let expected = "<plain>@markov@mldchan.dev</plain> <plain>@markov</plain>";
        let result = sanitize_mentions(mention.to_string());
        assert_eq!(result, expected);
    }
}
