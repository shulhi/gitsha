#![feature(slice_patterns)]

use std::error::Error;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

extern crate clap;
use clap::{App, Arg, SubCommand};

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
    let matches = App::new("GitSHA")
        .version("1.0")
        .author("Shulhi Sapli <shulhi@gmail.com>")
        .subcommand(
            SubCommand::with_name("get")
                .about("Get SHA")
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
                ),
        )
        .subcommand(
            SubCommand::with_name("configure")
                .about("Configuration")
                .arg(
                    Arg::with_name("TOKEN")
                        .help("Set Github API token")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("get") {
        let url = matches.value_of("REPO").unwrap();
        let branch = matches.value_of("branch").unwrap_or("master");

        if let Some((owner, repo)) = parse_owner_repo(url) {
            match env::home_dir() {
                None => println!("$HOME is not set"),
                Some(mut path) => {
                    path.push(".config");
                    path.push("gitc");
                    path.push("config");

                    match File::open(&path) {
                        Err(_why) => println!("Couldn't read configuration to get token."),
                        Ok(mut file) => {
                            let mut token = String::new();
                            match file.read_to_string(&mut token) {
                                Err(why) => panic!(
                                    "Couldn't read {}: {}",
                                    path.display(),
                                    why.description()
                                ),
                                Ok(_) => {
                                    let client = Github::new(token).unwrap();

                                    let commit =
                                        get_commit_information(&client, owner, repo, branch);

                                    match commit {
                                        Ok(c) => println!("{}", c.sha),
                                        Err(err) => println!("{}", err),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("configure") {
        let token = matches.value_of("TOKEN").unwrap();

        match env::home_dir() {
            Some(mut path) => {
                path.push(".config");
                path.push("gitc");

                fs::create_dir_all(&path).unwrap_or_else(|why| {
                    println!("! {:?}", why.kind());
                });

                // file name
                path.push("config");

                let mut file = match File::create(&path) {
                    Err(why) => panic!("couldn't create {}: {}", path.display(), why.description()),
                    Ok(file) => file,
                };

                match file.write_all(token.as_bytes()) {
                    Err(why) => panic!("Couldn't create {}: {}", path.display(), why.description()),
                    Ok(_) => println!("Wrote config to {}", path.display()),
                }
            }
            None => println!("$HOME is not set"),
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
        Err(_) => Err("Unknown error"),
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
