use glob::glob;

fn find_files(pattern: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    for entry in glob(pattern)? {
        let path = entry?;
        files.push(path.to_string_lossy().to_string());
    }

    Ok(files)

}


fn main() {
    let batch_files = match find_files("/Users/charlierothschild/Desktop/internship_projects/EC-developer-analysis/data/processed/commit_histories*/batch_*.json") {
        Ok(files) => {
            println!("Found {} files", files.len());
            files
        }
        Err(e) => {
            eprintln!("Error finding files: {}", e);
            Vec::new()
        }
    };

    for (idx, path) in batch_files.into_iter().take(10).enumerate() {
        println!("File {}: {}", idx, path);
    }
    
}

