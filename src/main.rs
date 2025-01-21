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

    let yaml_blocks = parse_spec_file(&args.spec_path).unwrap();


    // Print or use the extracted YAML blocks

    for (name, block) in yaml_blocks.iter() {
        println!("YAML Block {}-\n{}", name, block);
    }

    Ok(())
    
}
fn parse_spec_file(spec_path: &std::path::Path) -> io::Result<Vec<(String, String)>> {
    let file = File::open(spec_path)?;
    let reader = io::BufReader::new(file);

    let mut is_in_yaml_block = false;
    let mut yaml_blocks = Vec::new();
    let mut current_block = String::new();
    let mut current_block_name = String::new();

    let start_regex = Regex::new(r"^```yaml #s(\d+)").unwrap();

    for line in reader.lines() {
        let line = line?;

        if let Some(captures) = start_regex.captures(line.trim()) {
            is_in_yaml_block = true;
            current_block.clear();
            current_block_name = captures[1].to_string();
            continue;
        }

        if is_in_yaml_block && line.trim() == "```" {
            is_in_yaml_block = false;
            yaml_blocks.push((current_block_name.clone(), current_block.clone()));
            continue;
        }

        if is_in_yaml_block {
            current_block.push_str(&line);
            current_block.push('\n');
        }
    }

    Ok(yaml_blocks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_yaml_blocks() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "```yaml #s1\nkey: value\n```").unwrap();
        writeln!(file, "```yaml #s2\nanother_key: another_value\n```").unwrap();

        let args = Args {
            spec_path: file.path().to_path_buf(),
            example_path: std::path::PathBuf::new(),
        };

        let result = parse_spec_file(&args.spec_path).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "1");
        assert_eq!(result[0].1, "key: value\n");
        assert_eq!(result[1].0, "2");
        assert_eq!(result[1].1, "another_key: another_value\n");
    }

    #[test]
    fn test_no_yaml_blocks() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "This is a test file with no YAML blocks.").unwrap();

        let args = Args {
            spec_path: file.path().to_path_buf(),
            example_path: std::path::PathBuf::new(),
        };

        let result = parse_spec_file(&args.spec_path).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_incomplete_yaml_block() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "```yaml #s1\nkey: value").unwrap();

        let args = Args {
            spec_path: file.path().to_path_buf(),
            example_path: std::path::PathBuf::new(),
        };

        let result = parse_spec_file(&args.spec_path).unwrap();
        assert_eq!(result.len(), 0);
    }

}
