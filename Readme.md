# SnipComp

SnipComp is a Rust-based command-line tool designed to be used with [TOSCA Specs](https://github.com/oasis-tcs/tosca-specs)

It looks for YAML blocks called snippets from with a TOSCA Spec markdown file and replaces them with snippets extracted from the supplied examples directory having the same ID. The result is sent to stdout. A user may then use standard compare tools to see where spec snippets differ from example snippets and update the spec as required.

The program uses a regular expression to identify the start of YAML blocks, which are marked by lines beginning with "```yaml #s<number>". The examples are expected to be in files named with the ID and the example snippets are marked at the start with tags of the format # tag::s1[] and at the end with # end::s1[], spaces before and after the '#' character are permitted.

## Distribution
when complete the intention is to move this repo into a new directory within the repo [TOSCA Specs](https://github.com/oasis-tcs/tosca-specs)

## Installation

Option 1
Install Rust.

Clone this repo to folder.

cd to the folder and use cargo to run it:
```sh
cargo run -- --help
```

Option 2
Ensure the target execution environment is linux x86-64

Download the executable from [https://github.com/pmjordan/snipcomp/releases](https://github.com/pmjordan/snipcomp/releases)

Make the file executable and then run it:


## Usage
```sh
cargo run snipcomp --help
```

Typically
```sh
cargo run snipcomp -- -s examples/testspec.md -e toscaexamples/ > out.md
```

Alternatively, to get a report of which snippets do not match the examples (given that all examples exist) use
```sh
cargo run snipcomp -- -s examples/testspec.md -e toscaexamples/ -o report
```

## Directory Structure
In [TOSCA Specs](https://github.com/oasis-tcs/tosca-specs) the examples containing the snippets are held in a directory called examples but that folder name is normally used in Rust projects for examples relevant to the code and its tests so here the example snippets are held in a directory called toscaexamples.

## License

This project will be released under the same license as [TOSCA Specs](https://github.com/oasis-tcs/tosca-specs)

## Contact

For any questions or suggestions, please open an issue on the GitHub repository.
