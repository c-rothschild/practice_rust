
mod generate_user_dict;

use practice_rust::models::User;
use generate_user_dict::add_to_user_dict;

use std::fs;
use csv::Writer;
use std::collections::HashMap;

use practice_rust::{find_files, save_to_json};




fn main() {
   let batch_files = match find_files("/Volumes/CHARLIE_SSD/commit_histories*/batch_*.json") {
        Ok(files) => {
            println!("Found {} files", files.len());
            files
        }
        Err(e) => {
            eprintln!("error finding files: {}", e);
            return
        }
    };

    let mut user_dict = HashMap::<String, User>::new();

    for filepath in batch_files.iter().take(3) {
        println!("Processing: {}", filepath);
        match add_to_user_dict(filepath, &mut user_dict){
            Ok(_) => println!("Successfully processed {}", filepath),
            Err(e) => eprintln!("Error processing {}: {}", filepath, e),

        }
    }

    let filepath = "data/user_dict.json";

    match save_to_json(&user_dict, filepath) {
        Ok(_) => println!("Successfully saved to {}", filepath),
        Err(e) => eprintln!("Error saving file: {}", e),
    }
}