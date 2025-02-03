use std::{
    env, fs,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    posting_token: String,
    instance: String,
    accounts: Vec<PostsAccount>,
}

#[derive(Serialize, Deserialize)]
pub struct PostsAccount {
    pub id: String,
    pub token: String,
}

fn read_config() -> Config {
    // Read from file
    let mut config = String::new();

    fs::File::open("config.yml")
        .expect("Could not open config.yml file")
        .read_to_string(&mut config)
        .expect("Could not read config.yml file");

    serde_yml::from_str(&config).expect("Could not parse config")
}

pub fn read_posting_token() -> String {
    let config = read_config();

    config.posting_token
}

pub fn read_instance() -> String {
    let config = read_config();

    config.instance
}

pub fn read_accounts() -> Vec<PostsAccount> {
    let config = read_config();

    config.accounts
}
