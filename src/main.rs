//use assert_cmd::output;
use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Parse spec file for YAML snippets, substitute a matching snippet taken from in the examples directory and send result to stdout",
    long_about = "Snippets in the spec are identified by a comment line with the format ```yaml #sN where N is an integer
Examples are in files named with snippet number and a .yaml extension, e.g. s1.yaml
The snippet within the example is identified by a open and closing comment line with the format # tag::s2[] and # end::s2[]"
)]
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

    let file = File::open(&args.spec_path).map_err(|error| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Error opening spec file '{}': {}", args.spec_path.display(), error),
        )
    })?;
    let reader = io::BufReader::new(file);

    let mut is_in_yaml_block = false;
    let mut current_block = String::new();
    let mut current_block_name = String::new();
    let mut my_output = String::new();

    let start_regex = Regex::new(r"^```yaml #s(\d+)").unwrap();

    for line in reader.lines() {
        let line = line?;

        if let Some(captures) = start_regex.captures(line.trim()) {
            // Start of a new yaml block so add to output
            my_output.push_str(&line);my_output.push('\n');

            if is_in_yaml_block {
                if line.trim() == "```" {
                    is_in_yaml_block = false;
                } else {
                    if is_in_yaml_block {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Unclosed YAML block before text'{}'", line),
                        ));
                    }
                }
                continue;
            }

            is_in_yaml_block = true;
            current_block.clear();
            current_block_name = captures[1].to_string();
            continue;
        }

        if is_in_yaml_block && line.trim() == "```" {
            // End of the current block so substitute snippet
            let snippet = get_example_snippet(&args.example_path, &current_block_name)?;
            my_output.push_str(&snippet);
            // And output the end marker
            my_output.push_str(&line);my_output.push('\n');
            is_in_yaml_block = false;
            continue;
        }

        if is_in_yaml_block {
            current_block.push_str(&line);
            current_block.push('\n');
        }
        // Not in yaml block so add to output
        else {
            my_output.push_str(&line);my_output.push('\n');
        }
    }

    if is_in_yaml_block {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unclosed YAML block before end of file"),
        ));
    }

    write_output(&my_output)?;

    Ok(())
}

fn get_example_snippet(directory: &std::path::Path, snip_id: &str) -> io::Result<String> {
    let file_path = directory.join(format!("s{}.yaml", snip_id));
    let mut example_file = File::open(&file_path).map_err(|error| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Error opening file '{}': {}", file_path.display(), error),
        )
    })?;
    let mut content = String::new();
    example_file.read_to_string(&mut content)?;

    let start_tag_pattern = Regex::new(&format!(r"# +tag::s{}\[\]", snip_id)).unwrap();
    let end_tag_pattern = Regex::new(&format!(r"# +end::s{}\[\]", snip_id)).unwrap();

    let mut in_snippet = false;
    let mut snippet_content = String::new();

    for line in content.lines() {
        if start_tag_pattern.is_match(line.trim()) {
            in_snippet = true;
            continue;
        }
        if end_tag_pattern.is_match(line.trim()) {
            //We don't care what comes after the end tag
            break;
        }
        if in_snippet {
            snippet_content.push_str(line);
            snippet_content.push('\n');
        }
    }

    if snippet_content.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Snippet {} not found in file {}", snip_id, file_path.display()),
        ));
    }

    Ok(snippet_content)
}

fn write_output(output: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(output.as_bytes())
}

#[cfg(test)]
mod tests {

    use std::path::Path;
    use super::*;

    #[test]
    fn test_snippet_found() {
        let examples_dir = Path::new("examples");
        let result = get_example_snippet(examples_dir, "1").unwrap();
        assert_eq!(result, "text in snippet:\n  last row of snippet:\n");
    }

    #[test]
    fn test_example_directory_not_found() {
        let examples_dir = Path::new("not_there");
        let result = get_example_snippet(examples_dir, "1");
        assert!(
            result.unwrap_err().to_string().contains("Error opening file 'not_there/s1.yaml': No such file or directory"),
        );
    }

    #[test]
    fn test_example_snippet_not_started() {
        let examples_dir = Path::new("examples");
        let result = get_example_snippet(examples_dir, "20");
        assert!(
            result.unwrap_err().to_string().contains("Snippet 20 not found in file examples/s20.yaml"),
        );
    }

    #[test]
    fn test_example_snippet_incomplete() {
        let examples_dir = Path::new("examples");
        let result = get_example_snippet(examples_dir, "10");
        assert!(
            result.unwrap_err().to_string().contains("Snippet 10 not found in file examples/s10.yaml"),
        );
    }

    #[test]
    fn test_example_snippet_wrong_number() {
        let examples_dir = Path::new("examples");
        let result = get_example_snippet(examples_dir, "30");
        assert!(
            result.unwrap_err().to_string().contains("Snippet 30 not found in file examples/s30.yaml"),
        );
    }

    #[test]
    fn test_snippet_with_space_found() {
        let examples_dir = Path::new("examples");
        let result = get_example_snippet(examples_dir, "40").unwrap();
        assert_eq!(result, "text in snippet:\n  last row of snippet. end tag has space:\n");
    }
} 

