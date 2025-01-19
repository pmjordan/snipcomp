use std::env;
use std::fs::File;
use std::io::{self, BufRead};
//use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    // Get the spec path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <spec_path>", args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Spec path not provided"));
    }

    if args[1] == "-h" {
        println!("Usage: {} <spec_path>", args[0]);
        println!("Extract YAML blocks from a markdown file with sections starting with \"```yaml #s<number>\".");
        return Ok(());
    }

    let spec_path = &args[1];

    // Open the file with error handling
    let file = match File::open(&spec_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Could not open file '{}': {}", spec_path, e);
            return Err(e);
        }
    };

    let reader = io::BufReader::new(file);

    let mut is_in_yaml_block = false;
    let mut yaml_blocks = Vec::new();
    let mut current_block = String::new();
    let mut current_block_name = String::new();

    // Create a regex to match the start of a YAML block with an integer
    let start_regex = Regex::new(r"^```yaml #s(\d+)").unwrap();

    for line in reader.lines() {
        let line = line?;

        // Check for the start of a YAML block
        if let Some(captures) = start_regex.captures(line.trim()) {
            is_in_yaml_block = true;
            current_block.clear();
            current_block_name = captures[1].to_string(); // Extract the integer
            continue;
        }

        // Check for the end of a YAML block
        if is_in_yaml_block && line.trim() == "```" {
            is_in_yaml_block = false;
            yaml_blocks.push((current_block_name.clone(), current_block.clone()));
            continue;
        }

        // Collect lines within the YAML block
        if is_in_yaml_block {
            current_block.push_str(&line);
            current_block.push('\n');
        }
    }

    // Print or use the extracted YAML blocks
    for (name, block) in yaml_blocks.iter() {
        println!("YAML Block #{}:\n{}", name, block);
    }

    Ok(())
}

