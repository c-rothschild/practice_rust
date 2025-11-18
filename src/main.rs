use practice_rust::models::GithubGraph;
use std::collections::HashMap;
use practice_rust::models::CommitDictEntry;
use std::fs;
use serde_json;


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

fn main() {

    

    let filepath = "data/github_graph.json";

    let mut graph = match GithubGraph::load_from_file(filepath) {
        Ok(graph) => {
            println!("Succerssfully loaded graph from file!");
            graph
        },
        Err(e) => {
            eprintln!("Error loading graph: {}", e);
            GithubGraph::new()
        }
    };


    let filepath = "data/commit_dict1.json";

    match add_commit_dict_to_graph(&filepath, &mut graph) {
        Ok(_) => println!("Successfully added commit dict to graph"),
        Err(e) => {
            eprintln!("Error adding commit dict to graph: {}", e);
            return
        }
    };


    let filepath = "data/github_graph.json";

    match graph.save_to_file(&filepath){
        Ok(_) => println!("Graph successfully saved to {}", filepath),
        Err(e) => println!("Error saving graph: {}", e),
    }


}
