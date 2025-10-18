mod models;

use models::*;

use serde_json;
use std::fs;
use glob::glob;

use std::collections::HashMap;


fn load_repo_file(filepath: &str) -> Result<RepositoryData, Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string(filepath)?;
    let repo_data: RepositoryData = serde_json::from_str(&json_str)?;
    Ok(repo_data)
}

fn find_files(pattern: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    for entry in glob(pattern)? {
        let path = entry?;
        files.push(path.to_string_lossy().to_string());
    }

    Ok(files)

}

// add commits from a filepath to our commit count dict
fn add_to_commit_count(filepath: &str, commit_counts: &mut HashMap<String, UserCommitCounts>) {

    let repo_data = match load_repo_file(filepath) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading repos: {}", e);
            return;
        }
    };

    // loop through each repo in repo_data
    for (repo_name, commits) in repo_data {
        if commits.is_empty() {
            continue;
        }

        // loop through each commit in the repo
        for commit in commits {

            let author_login = match &commit.author {
                Some(user) => {
                    match &user.login {
                        Some(login) => login,
                        None => {
                            continue;
                        }
                    }
                }
                None => {
                    // Skip commits without author info
                    continue;
                }
            };

            // Modify the author's user_commit_counts entry

            let user_stats = commit_counts
                .entry(author_login.clone())
                .or_insert(UserCommitCounts {
                    total_commits: 0,
                    repo_commits: HashMap::new(),
                });
            
            user_stats.total_commits += 1;
            
            *user_stats.repo_commits
                .entry(repo_name.clone())
                .or_insert(0) += 1;

        }

    }



}

fn save_to_json(commit_counts: &HashMap<String, UserCommitCounts>, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string_pretty(commit_counts)?;
    fs::write(filepath, json_string)?;
    Ok(())
}


fn main() {
    let batch_files = match find_files("/Users/charlierothschild/Desktop/internship_projects/EC-developer-analysis/data/processed/commit_histories*/batch_*.json") {
        Ok(files) => {
            println!("Found {} files", files.len());
            files
        }
        Err(e) => {
            eprintln!("Error finding files: {}", e);
            return
        }
    };
    let mut commit_counts = HashMap::<String, UserCommitCounts>::new();

    for filepath in batch_files.iter() {
        println!("Processing: {}", filepath);
        add_to_commit_count(filepath, &mut commit_counts);
    };

    for (idx, (author, stats)) in commit_counts.iter().take(3).enumerate() {
        println!("{}. {}: {} total commits", idx + 1, author, stats.total_commits);

        for (repo, count) in &stats.repo_commits {
            println!("     {}: {} commits", repo, count);
        }
    };

    let filepath = "data/commit_counts.json";

    match save_to_json(&commit_counts, &filepath) {
        Ok(_) => println!("Successfully saved to {}", filepath),
        Err(e) => eprintln!("Error saving file: {}", e),
    }

    
    

}