pub mod models;
pub mod generate_user_dict;

use serde_json;
use glob::glob;
use std::fs;
use serde::Serialize;
use models::RepoDicts;


pub fn load_repo_file(filepath: &str) -> Result<RepoDicts, Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string(filepath)?;
    let repo_data: RepoDicts = serde_json::from_str(&json_str)?;
    Ok(repo_data)
}

pub fn save_to_json<T>(data: &T, filepath: &str) -> Result<(), Box<dyn std::error::Error>> 
where
    T: Serialize,
{
    let json_string: String = serde_json::to_string_pretty(data)?;
    fs::write(filepath, json_string)?;
    Ok(())
}


pub fn find_files(pattern: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    for entry in glob(pattern)? {
        let path = entry?;
        files.push(path.to_string_lossy().to_string());
    }

    Ok(files)

}