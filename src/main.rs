use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(version, 
about = "Parse spec file for YAML snippets and check there is a matching snippet in the examples directory", 
long_about = "Snippets in the spec are identified by a comment line with the format ```yaml #sN where N is an integer
Examples are in files named with snippet number and a .yaml extension, e.g. s1.yaml
The snippet within the example is identified by a open and closing comment line with the format # tag::s2[] and # end::s2[]")]
struct Args {
    /// The path to the spec file
    #[arg(short, long)]
    spec_path: std::path::PathBuf,
    /// The path to the example directory
    #[arg(short, long)]
    example_path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    // Get the spec path from command-line arguments
    let args = Args::parse();

    let spec_path = args.spec_path;

    // Open the file with error handling
    let file = match File::open(&spec_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Could not open file '{:?}': {}", spec_path, e);
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
        println!("YAML Block {}-\n{}", name, block);
    }

    Ok(())
}
