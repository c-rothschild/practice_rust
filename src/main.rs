use serde_json;
use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//Main structure - a map of repo names to commit arrays
type RepositoryData = HashMap<String, Vec<Commit>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub node_id: String,
    pub commit: CommitDetails,
    pub url: String,
    pub html_url: String,
    pub comments_url: String,
    pub author: Option<User>,
    pub committer: Option<User>,
    pub parents: Vec<Parent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: CommitAuthor,
    pub committer: CommitAuthor,
    pub message: String,
    pub tree: Tree,
    pub url: String,
    pub comment_count: u32,
    pub verification: Verification,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub signature: Option<String>,
    pub payload: Option<String>,
    pub verified_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub user_view_type: String,
    pub site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parent {
    pub sha: String,
    pub url: String,
    pub html_url: String
}

fn load_repos(filepath: &str) -> Result<RepositoryData, Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string(filepath)?;
    let repo_data: RepositoryData = serde_json::from_str(&json_str)?;
    Ok(repo_data)
}


fn main() {
    let json_str = fs::read_to_string("/Users/charlierothschild/Desktop/internship_projects/EC-developer-analysis/data/processed/github_repos.json").expect("failed to read file");    
    
    let repo_list: Vec<String> = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    for (i, item) in repo_list.iter().take(10).enumerate() {
        println!("{}: {}", i + 1, item);
    }

    // Loading batch_378.json
    let repo_data = match load_repos("data/batch_378.json") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading repos: {}", e);
            return
        }
    };
    // print the first 10 keys
    for (idx, key) in repo_data.keys().take(10).enumerate() { 
        println!("{}: {}", idx, key);
    }


}