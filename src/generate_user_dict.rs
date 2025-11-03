

use serde_json;
use practice_rust::models::User;
use std::collections::HashMap;
use std::fs;
use std::error::Error;
use practice_rust::load_repo_file;

pub fn add_to_user_dict(filepath: &str, user_dict: &mut HashMap<String, User>) -> Result<(), Box<dyn Error>> {
    let repo_data = match load_repo_file(filepath) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading repos: {}", e);
            return Ok(());
        }

    };

    for (_repo_name, commits) in &repo_data {
        for commit in commits {
            // add author to dict
            if let Some(author) = &commit.author {
                if let Some(author_login) = &author.login {
                    user_dict.entry(author_login.clone()).or_insert_with(|| (*author).clone());
                }
            }
            // add committer to dict
            if let Some(committer) = &commit.committer {
                if let Some(committer_login) = &committer.login {
                    user_dict.entry(committer_login.clone()).or_insert_with(|| (*committer).clone());
                }
            }
        }
    }
    Ok(())

}