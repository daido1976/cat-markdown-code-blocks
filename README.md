# cat-markdown-code-blocks

`cat-markdown-code-blocks` is a CLI tool that reads multiple files and concatenates their contents into Markdown-formatted code blocks. This tool is particularly useful for systems like ChatGPT, assisting in the quick sharing and documentation of file contents.

## Installation

To install `cat-markdown-code-blocks` locally, first clone the project to your local machine and navigate to the root directory of the project. Then run the following command:

```sh
$ cargo install --path ./cli
```

## Usage

```sh
$ cmcb -h
Usage: cmcb [OPTIONS] <FILES>...

Arguments:
  <FILES>...  Input files

Options:
  -c, --clipboard  Copy to clipboard
  -h, --help       Print help
  -V, --version    Print version
```
