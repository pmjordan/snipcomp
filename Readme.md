# SnipComp

SnipComp is a Rust-based command-line tool designed to be used with [TOSCA Specs](https://github.com/oasis-tcs/tosca-specs)

It extracts YAML blocks called snippets from a TOSCA Spec markdown file and compares them to snippets in the examples directory

The program uses a regular expression to identify the start of YAML blocks, which are marked by lines beginning with "```yaml #s<number>". 


## Installation

To install SnipComp, ensure you have Rust installed, then run:

```sh
cargo install snipcomp
```
## Usage
```sh
cargo run snipcomp --help
```

## License

This project will be released under the same license as [TOSCA Specs](https://github.com/oasis-tcs/tosca-specs)

## Contact

For any questions or suggestions, please open an issue on the GitHub repository.
