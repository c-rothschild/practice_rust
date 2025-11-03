mod models;

use models::UserCommitCounts;

use serde_json;
use std::fs;
use glob::glob;
use csv::Writer;
use std::fs::File;

use std::collections::HashMap;

use practice_rust::{load_repo_file, find_files}






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

fn convert_json_to_csv(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read and parse the JSON file
    let json_str = fs::read_to_string(json_path)?;
    let commit_counts: HashMap<String, UserCommitCounts> = serde_json::from_str(&json_str)?;
    
    // Create CSV writer
    let mut wtr = Writer::from_path(csv_path)?;
    
    // Write header
    wtr.write_record(&["Username", "Total Commits", "Total Repos", "Repo Contributions"])?;
    
    // Write data rows
    for (username, stats) in &commit_counts {
        // Calculate total repos
        let total_repos = stats.repo_commits.len();
        
        // Format repo contributions as "repo1: 5, repo2: 10, ..."
        let repo_details: Vec<String> = stats.repo_commits
            .iter()
            .map(|(repo, count)| format!("{}: {}", repo, count))
            .collect();
        let repo_contributions = repo_details.join(", ");
        
        // Write row
        wtr.write_record(&[
            username,
            &stats.total_commits.to_string(),
            &total_repos.to_string(),
            &repo_contributions,
        ])?;
    }
    
    wtr.flush()?;
    println!("Successfully wrote CSV to {}", csv_path);
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