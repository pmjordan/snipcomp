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

    let file = File::open(&args.spec_path)?;
    let reader = io::BufReader::new(file);

    let mut is_in_yaml_block = false;
    let mut current_block = String::new();
    let mut current_block_name = String::new();
    let mut my_output = String::new();

    let start_regex = Regex::new(r"^```yaml #s(\d+)").unwrap();

    for line in reader.lines() {
        let line = line?;

        if let Some(captures) = start_regex.captures(line.trim()) {
            // Start of a new yaml block so send to stdout
            //println!("{}", line);
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
            //print!("{}",snippet);
            my_output.push_str(&snippet);
            // And output the end marker
            //println!("{}", line);
            my_output.push_str(&line);my_output.push('\n');
            is_in_yaml_block = false;
            continue;
        }

        if is_in_yaml_block {
            current_block.push_str(&line);
            current_block.push('\n');
        }
        // Not in yaml block so send to stdout
        else {
            //println!("{}", line);
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
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
//TODO allow any number of spaces before the tag and between hash and tag
    let start_tag = format!("# tag::s{}[]", snip_id);
    let end_tag = format!("# end::s{}[]", snip_id);

    let mut in_snippet = false;
    let mut snippet_content = String::new();

    for line in content.lines() {
        if line.trim() == start_tag {
            in_snippet = true;
            continue;
        }
        if line.trim() == end_tag {
            in_snippet = false;
            break;
            //TODO error if end tag not found
        }
        if in_snippet {
            snippet_content.push_str(line);
            snippet_content.push('\n');
        }
    }

    if snippet_content.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Snippet {} not found in file", snip_id),
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
    use assert_cmd::assert::AssertError;

    use super::*;
    use std::io::Write;


    #[test]
    fn test_example_snippet_found() {
        let file_path = std::path::Path::new("s1.yaml");
        {
            let mut file = File::create(file_path).unwrap();
            writeln!(file, "text before tag\n     # tag::s1[]\ntext in snippet\nlast row of snippet\n# end::s1[]\ntext after snippet").unwrap();
        }

        let result = get_example_snippet(file_path.parent().unwrap(), "1").unwrap();
        assert_eq!(result, "text in snippet\nlast row of snippet\n");

        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_example_snippet_not_found() {
        let file_path = std::path::Path::new("s1.yaml");
        {
            let mut file = File::create(file_path).unwrap();
            writeln!(file, "# tag::s1[]\nexample_key: example_value\n# end::s1[]").unwrap();
        }
        let result = get_example_snippet(file_path.parent().unwrap(), "2").unwrap();
        //TODO assert something

        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_example_snippet_incomplete() {
        let file_path = std::path::Path::new("s1.yaml");
        {
            let mut file = File::create(file_path).unwrap();
            writeln!(file, "# tag::s1[]\nexample_key: example_value").unwrap();
        }
        let result = get_example_snippet(file_path.parent().unwrap(), "1").unwrap();
        //TODO assert error

        std::fs::remove_file(file_path).unwrap();
    }
}
