use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//Main structure - a map of repo names to commit arrays
pub type RepositoryData = HashMap<String, Vec<Commit>>;

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
    pub login: Option<String>,         // Make optional
    pub id: Option<u64>,                // Make optional
    pub node_id: Option<String>,        // Make optional
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    pub r#type: Option<String>,
    pub user_view_type: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parent {
    pub sha: String,
    pub url: String,
    pub html_url: String
}

#[derive(Debug, Serialize)]
pub struct UserCommitCounts {
    pub total_commits: u32,
    pub repo_commits: HashMap<String, u32>,
}