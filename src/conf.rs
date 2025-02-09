use std::{fs, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    posting_token: String,
    instance: String,
    accounts: Vec<PostsAccount>,
    testing: Option<TestingConfiguration>
}

#[derive(Serialize, Deserialize)]
pub struct PostsAccount {
    pub id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct TestingConfiguration {
    pub disable_post: Option<bool> // Test configuration to disable posting when testing functionality
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

pub fn read_testing_config() -> Option<TestingConfiguration> {
    let config = read_config();

    config.testing
}

pub fn read_disable_post() -> bool {
    let config = read_testing_config();
    if let Some(testing_config) = config {
        if let Some(disable_post) = testing_config.disable_post {
            return disable_post;
        }
    }

    false
}

#[cfg(test)]
#[serial_test::serial]
mod tests {
    use super::*;

    #[test]
    fn test_read_config() {
        // Write testing configuration
        let config = r#"
posting_token: token
instance: misskey.io
accounts:
  - id: 1234567890
    token: token
"#;

        fs::write("config.yml", config).unwrap();

        let config = read_config();
        assert_eq!(config.posting_token, "token");
        assert_eq!(config.instance, "misskey.io");
        assert_eq!(config.accounts.len(), 1);
        assert_eq!(config.accounts[0].id, "1234567890");
        assert_eq!(config.accounts[0].token, "token");

        let posting_token = read_posting_token();
        assert_eq!(posting_token, "token");

        let instance = read_instance();
        assert_eq!(instance, "misskey.io");

        let accounts = read_accounts();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].id, "1234567890");
        assert_eq!(accounts[0].token, "token");

        let testing_config = read_testing_config();
        assert_eq!(testing_config.is_none(), true);

        let disable_post = read_disable_post();
        assert_eq!(disable_post, false);
    }
}
