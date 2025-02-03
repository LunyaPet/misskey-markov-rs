use std::{
    env, fs,
    io::{Read, Write},
};

pub fn read_token() -> String {
    // Check environment variable
    if let Ok(token) = env::var("MISSKEY_TOKEN") {
        return token;
    }

    // Read from file
    let mut token = String::new();

    fs::File::open("token.txt")
        .expect("Could not open file")
        .read_to_string(&mut token)
        .expect("Could not read file");

    token.trim().to_string()
}

pub fn write_token(token: &str) {
    // Write to file
    fs::File::create("token.txt")
        .expect("Could not create file")
        .write_all(token.as_bytes())
        .expect("Could not write to file");
}

pub fn read_instance() -> String {
    // Check environment variable
    if let Ok(instance) = env::var("MISSKEY_INSTANCE") {
        return instance;
    }

    // Read from file
    let mut instance = String::new();

    fs::File::open("instance.txt")
        .expect("Could not open file")
        .read_to_string(&mut instance)
        .expect("Could not read file");

    instance.trim().to_string()
}

pub fn write_instance(instance: &str) {
    // Write to file
    fs::File::create("instance.txt")
        .expect("Could not create file")
        .write_all(instance.as_bytes())
        .expect("Could not write to file");
}
