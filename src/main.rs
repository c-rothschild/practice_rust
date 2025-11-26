use practice_rust::models::GithubGraph;
use std::collections::HashMap;
use practice_rust::models::CommitDictEntry;
use std::fs;
use serde_json;
use practice_rust::{find_files, load_repo_file, save_to_json};
use practice_rust::generate_user_dict::add_to_commit_dict;

fn load_commit_dict(filepath: &str) -> Result<HashMap<String, CommitDictEntry>, Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string(filepath)?;
    println!("Successfully read file!");
    let commit_dict: HashMap<String, CommitDictEntry> = serde_json::from_str(&json_str)?;
    println!("Successfully parsed file!");
    Ok(commit_dict)
}

fn add_commit_to_graph(entry: &CommitDictEntry, graph: &mut GithubGraph){
    let user_login = entry.author.clone();
    let repo_name = entry.repo_name.clone();
    let sha = entry.sha.clone();
    graph.add_commit_to_graph(&user_login, &repo_name, &sha);
}


fn add_commit_dict_to_graph(filepath: &str, graph: &mut GithubGraph) -> Result<(), Box<dyn std::error::Error>> {
    let commit_dict = load_commit_dict(filepath)?;

    for entry in commit_dict.values() {
        add_commit_to_graph(entry, graph);
        // println!("Commit {} added to graph", entry.sha)
    }

    Ok(())
}

fn add_file_to_forked_dict( filepath: &str, is_forked_dict: &mut HashMap<String, bool>) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = fs::read_to_string(filepath)?;
    let new_forked_dict: HashMap<String, Option<bool>> = serde_json::from_str(&json_string)?;

    let converted_dict: HashMap<String, bool> = new_forked_dict
        .into_iter()
        .map(|(key, value)| (key, value.unwrap_or(true)))
        .collect();

    // Add all items from new_forked_dict to is_forked_dict
    is_forked_dict.extend(converted_dict);

    Ok(())
}

fn main() {
    // getting top 20k non-forked repos
    // open forked dict
    let filepath = "/Users/charlierothschild/Desktop/internship_projects/EC-developer-analysis/data/processed/is_forked_batch1.json";
    let mut is_forked_dict: HashMap<String, bool> = HashMap::new();

    match add_file_to_forked_dict(filepath, &mut is_forked_dict) {
        Ok(_) => println!("Successfully added file to forked dict"),
        Err(e) => {
            eprintln!("Error adding file to forked dict: {}", e);
            return
        }
    };

    let filepath = "/Users/charlierothschild/Desktop/internship_projects/EC-developer-analysis/data/processed/is_forked_batch2.json";

    match add_file_to_forked_dict(filepath, &mut is_forked_dict) {
        Ok(_) => println!("Successfully added file to forked dict"),
        Err(e) => {
            eprintln!("Error adding file to forked dict: {}", e);
            return
        }
    };

    // load sorted_commit

    let filepath = "data/sorted_commit_counts.json";

    let json_string = match fs::read_to_string(filepath) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading {}: {}", filepath, e);
            return
        }
    };

    let sorted_commits: Vec<(String, u32)> = match serde_json::from_str(&json_string) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error  {}: {}", filepath, e);
            return
        }
    };

    let mut top_unforked_repos: Vec<String> = Vec::new();

    let size: usize = 20000;

    for (repo_name, _) in sorted_commits.iter().rev(){
        let is_forked = is_forked_dict.get(repo_name).copied().unwrap_or(true);
        if is_forked == false {
            top_unforked_repos.push(repo_name.clone());
        }
        if top_unforked_repos.len() >= size{
            break;
        }
    }

    let filepath = "data/top_unforked_repos.json";

    match save_to_json(&top_unforked_repos, filepath) {
        Ok(_) => println!("Successfully saved top unforked repos"),
        Err(e) => eprintln!("Error saving unforked repos: {}", e),
    }
    










    // // Generate commit dicts
    // //load top repos
    // let filepath = "data/github_repos.json";
    // let json = match std::fs::read_to_string(filepath){
    //     Ok(data) => data,
    //     Err(e) => {
    //         eprintln!("Error loading file {}", e);
    //         return
    //     }
    // };
    // let top_repos: Vec<String> = match serde_json::from_str(&json){
    //     Ok(data) => data,
    //     Err(e) => {
    //         eprintln!("Error parsing json file: {}", e);
    //         return
    //     }
    // };



    

    // let batch_files = match find_files("/Volumes/CHARLIE_SSD/commit_histories3/batch_*.json") {
    //     Ok(data) => data,
    //     Err(e) => {
    //         eprintln!("Error finding files: {}", e);
    //         return
    //     }
    // };

    // let mut commit_dict: HashMap<String, CommitDictEntry> = HashMap::new();

    // for filepath in batch_files.iter(){
    //     match add_to_commit_dict(&filepath, &mut commit_dict, &top_repos) {
    //         Ok(_) => println!("Successfully added {} to file", filepath),
    //         Err(e) => eprintln!("Error adding file to filepath: {}", e),
    //     };
    // }

    // let filepath = "data/commit_dict3_2.json";
    // let json = match serde_json::to_string_pretty(&commit_dict){
    //     Ok(data) => data,
    //     Err(e) => {
    //         eprintln!("Error converting commit dict to json: {}", e);
    //         return
    //     }
    // };
    // match std::fs::write(filepath, json) {
    //     Ok(_) => println!("Successfully saved to {}", filepath),
    //     Err(e) => eprintln!("Error Saving file: {}", e),
    // }


    
    // Generate graph based on commit dicts

    // let filepath = "data/github_graph.json";

    // let mut graph = match GithubGraph::load_from_file(filepath) {
    //     Ok(graph) => {
    //         println!("Succerssfully loaded graph from file!");
    //         graph
    //     },
    //     Err(e) => {
    //         eprintln!("Error loading graph: {}", e);
    //         GithubGraph::new()
    //     }
    // };


    // let filepath = "data/commit_dict3.json";

    // match add_commit_dict_to_graph(&filepath, &mut graph) {
    //     Ok(_) => println!("Successfully added commit dict to graph"),
    //     Err(e) => {
    //         eprintln!("Error adding commit dict to graph: {}", e);
    //         return
    //     }
    // };


    // let filepath = "data/github_graph.json";

    // match graph.save_to_file(&filepath){
    //     Ok(_) => println!("Graph successfully saved to {}", filepath),
    //     Err(e) => println!("Error saving graph: {}", e),
    // }




}
