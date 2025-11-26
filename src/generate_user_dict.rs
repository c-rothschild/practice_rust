use crate::models::{User, CommitDictEntry, Commit};
use std::collections::HashMap;
use std::error::Error;
use crate::load_repo_file;

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

pub fn add_to_repo_commit_count_dict(filepath: &str, repo_commit_count_dict: &mut HashMap<String, u32>) -> Result<(), Box<dyn Error>>{

    let repo_data = match load_repo_file(filepath) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading repos: {}", e);
            return Ok(());
        }
    };
    for (repo_name, commits) in &repo_data {
        if commits.is_empty() {
            continue;
        }
        repo_commit_count_dict
            .entry(repo_name.clone())
            .or_insert(commits.len() as u32);
    }
    Ok(())
}



pub fn add_to_commit_dict(filepath: &str, commit_dict: &mut HashMap<String, CommitDictEntry>, top_repos: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let repo_data = load_repo_file(filepath)?;
    for (repo_name, commits) in &repo_data {
        if !top_repos.contains(repo_name) {
            continue;
        }
        for commit in commits.iter() {
            let commit_key = format!("{}:{}", repo_name, commit.sha);
            let commit_dict_entry = commit.to_dict_entry(repo_name);

            commit_dict.entry(commit_key).or_insert(commit_dict_entry);


        }
    }

    Ok(())
}