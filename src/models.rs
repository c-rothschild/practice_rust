use petgraph::graph::{Graph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use petgraph::Undirected;
use serde_json;


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NodeKind {
    User,
    Repo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeData {
    kind: NodeKind,
    name: String, //github login or "owner/repo"
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct EdgeData {
    weight: u32,
    shas: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct GithubGraph {
    graph: Graph<NodeData, EdgeData, Undirected>,
    index_by_name: HashMap<String, NodeIndex>,
}

impl GithubGraph {
    pub fn new() -> Self {
        Self {
            graph: Graph::new_undirected(),
            index_by_name: HashMap::new(),
        }
    }

    pub fn get_or_add_user(&mut self, login: &str) -> NodeIndex {
        self.get_or_add_node(login, NodeKind::User)
    }

    pub fn get_or_add_repo(&mut self, repo: &str) -> NodeIndex {
        self.get_or_add_node(repo, NodeKind::Repo)
    }

    pub fn get_or_add_node(&mut self, name: &str, kind: NodeKind) -> NodeIndex {
        if let Some(&idx) = self.index_by_name.get(name) {
            return idx;
        }
        let idx = self.graph.add_node(NodeData {
            kind,
            name: name.to_string(),
        });
        self.index_by_name.insert(name.to_string(), idx);
        idx
    }

    pub fn add_commit_to_graph(&mut self, user_login: &str, repo_name: &str, sha: &str) {
        let user_idx = self.get_or_add_user(user_login);
        let repo_idx = self.get_or_add_repo(repo_name);

        debug_assert_eq!(self.graph[user_idx].kind, NodeKind::User);
        debug_assert_eq!(self.graph[repo_idx].kind, NodeKind::Repo);

        //check if edge already exists
        if let Some(edge_idx) = self.graph.find_edge(user_idx, repo_idx) {
            //update existing edge
            let edge_data = self.graph.edge_weight_mut(edge_idx).unwrap();
            edge_data.weight += 1;
            edge_data.shas.push(sha.to_string());
        } else {
            // Create new edge
            let data = EdgeData {
                weight: 1,
                shas: vec![sha.to_string()],
            };
            self.graph.add_edge(user_idx, repo_idx, data);
        }
    }

    pub fn save_to_file(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>>{
        let json = serde_json::to_string_pretty(&self.graph)?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    pub fn load_from_file(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        let graph: Graph<NodeData, EdgeData, Undirected> = 
            serde_json::from_str(&json)?;
        
        // rebuild index_by_name hash map
        let mut index_by_name: HashMap<String, NodeIndex> = HashMap::new();
        for idx in graph.node_indices() {
            index_by_name.insert(graph[idx].name.clone(), idx);
        }

        Ok(Self { graph, index_by_name })
    }

    
}


//Main structure - a map of repo names to commit arrays
pub type RepoDicts = HashMap<String, Vec<Commit>>;

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
pub struct CommitDictEntry {
    pub repo_name: String,
    pub sha: String,
    pub author: String,
    pub committer: String,
    pub date: String,
    pub message: String,
    pub url: String,
    pub comment_count: u32,
}



impl Commit {
    pub fn to_dict_entry(&self, repo_name: &str) -> CommitDictEntry {
        CommitDictEntry { 
            repo_name: repo_name.to_string(), 
            sha: self.sha.clone(), 
            author: self.commit.author.name.clone(),
            committer: self.commit.committer.name.clone(),
            date: self.commit.committer.date.clone(),
            message: self.commit.message.clone(),
            url: self.url.clone(), 
            comment_count: self.commit.comment_count.clone(),
         }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitDetails {
    pub author: CommitAuthor,
    pub committer: CommitAuthor,
    pub message: String,
    pub tree: Tree,
    pub url: String,
    pub comment_count: u32,
    pub verification: Verification,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tree {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub signature: Option<String>,
    pub payload: Option<String>,
    pub verified_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parent {
    pub sha: String,
    pub url: String,
    pub html_url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCommitCounts {
    pub total_commits: u32,
    pub repo_commits: HashMap<String, u32>,
}
