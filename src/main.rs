#![feature(slice_patterns)]

extern crate clap;
use clap::{App, Arg};

extern crate github_rs;
extern crate serde_json;
use github_rs::StatusCode;
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
    let branch = matches.value_of("branch").unwrap_or("master");

    if let Some((owner, repo)) = parse_owner_repo(url) {
        let client = Github::new("4929d085b5afb5ee79781643a3cd4316e5da2b4e").unwrap();

        let commit = get_commit_information(&client, owner, repo, branch);

        match commit {
            Ok(c) => println!("{}", c.sha),
            Err(err) => println!("{}", err),
        }
    }
}

fn get_commit_information(
    client: &Github,
    owner: &str,
    repo_name: &str,
    branch_name: &str,
) -> Result<CommitInfo, &'static str> {
    let response = client
        .get()
        .repos()
        .owner(owner)
        .repo(repo_name)
        .branches()
        .name(branch_name)
        .execute::<Value>();

    match response {
        Ok((_, status, json)) => match status {
            StatusCode::Ok => json.ok_or("JSON is missing")
                .and_then(|v| parse_repo_info(&v)),
            StatusCode::NotFound => Err("Repo or branch is not found"),
            _ => Err("Unknown error"),
        },
        Err(e) => Err("Unknown error"),
    }
}

fn parse_repo_info(json: &Value) -> Result<CommitInfo, &'static str> {
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
            Ok(repo_info)
        }
        _ => Err("Parse error"),
    }
}

fn parse_owner_repo(input: &str) -> Option<(&str, &str)> {
    let results: Vec<&str> = input.split('/').collect();

    match &results[..] {
        &[owner, repo] => Some((owner, repo)),
        _ => None,
    }
}
