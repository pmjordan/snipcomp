# SnipComp

SnipComp is a Rust-based command-line tool designed to be used with [TOSCA Specs](https://github.com/oasis-tcs/tosca-specs)

It looks for YAML blocks called snippets from with a TOSCA Spec markdown file and replaces them with snippets extracted from the supplied examples directory having teh same ID. The result is sent to stdout. A user may then use standrd compare tools to see where spec snippets differ from example snippets and update teh spec as required.

The program uses a regular expression to identify the start of YAML blocks, which are marked by lines beginning with "```yaml #s<number>". The examples are expected to be in files named with the ID and the example snippets are marked at the start with tags of the format # tag::s1[] and at the end with # end::s1[]

## Distribution
when complete the intention is to move this repo into a new directory within the repo [TOSCA Specs](https://github.com/oasis-tcs/tosca-specs)

## Installation

Stage 1 (The only way supported so far)
Install Rust.
Clone this repo to folder 
cd to the folder
```sh
cargo run snipcomp --help
```

Stage 2 (Not yet implemented)
Install Rust

```sh
cargo install snipcomp
```
## Usage
```sh
cargo run snipcomp --help
```

Typically
```sh
cargo run snipcomp -- 
```

## License

This project will be released under the same license as [TOSCA Specs](https://github.com/oasis-tcs/tosca-specs)

## Contact

For any questions or suggestions, please open an issue on the GitHub repository.
