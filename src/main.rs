#![feature(slice_patterns)]

extern crate clap;
use clap::{App, Arg};

extern crate github_rs;
extern crate serde_json;
use github_rs::client::{Executor, Github};
use serde_json::Value;

struct CommitInfo {
    name: String,
    sha: String,
}

fn main() {
    let matches = App::new("Git commit retriever")
        .version("1.0")
        .author("Shulhi Sapli <shulhi@gmail.com>")
        .arg(
            Arg::with_name("REPO")
                .help("Use the following format <owner>/<repo>")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("branch")
                .help("default to master")
                .short("b")
                .long("branch")
                .takes_value(true),
        )
        .get_matches();

    let url = matches.value_of("REPO").unwrap();

    if let Some((owner, repo)) = parse_owner_repo(url) {
        let client = Github::new("4929d085b5afb5ee79781643a3cd4316e5da2b4e").unwrap();

        let commit = get_commit_information(&client, owner, repo);

        match commit {
            Some(c) => println!("{}", c.sha),
            None => println!("Nothing"),
        }
    }
}

fn get_commit_information(client: &Github, owner: &str, repo_name: &str) -> Option<CommitInfo> {
    let response = client
        .get()
        .repos()
        .owner(owner)
        .repo(repo_name)
        .branches()
        .name("master")
        .execute::<Value>();

    match response {
        Ok((_, _, json)) => json.and_then(|json| parse_repo_info(&json)),
        Err(e) => None,
    }
}

fn parse_repo_info(json: &Value) -> Option<CommitInfo> {
    let name = match json["name"] {
        Value::String(ref v) => Ok(v.clone()),
        _ => Err("Name not found"),
    };

    let sha = match json["commit"]["sha"] {
        Value::String(ref v) => Ok(v.clone()),
        _ => Err("Commit sha not found"),
    };

    match (name, sha) {
        (Ok(name), Ok(sha)) => {
            let repo_info = CommitInfo { name, sha };
            Some(repo_info)
        }
        _ => None,
    }
}

fn parse_owner_repo(input: &str) -> Option<(&str, &str)> {
    let results: Vec<&str> = input.split('/').collect();

    match &results[..] {
        &[owner, repo] => Some((owner, repo)),
        _ => None,
    }
}
