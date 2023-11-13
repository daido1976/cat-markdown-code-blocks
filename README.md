# cat-markdown-code-blocks

`cat-markdown-code-blocks` is a CLI tool that reads multiple files and concatenates their contents into Markdown-formatted code blocks. This tool is particularly useful for systems like ChatGPT, assisting in the quick sharing and documentation of file contents.

**Note**: A VSCode extension for `cat-markdown-code-blocks` is currently in development, but it is still a work in progress.

## Installation

There are two ways to install `cat-markdown-code-blocks`:

### From GitHub

To install directly from the GitHub repository, use the following command:

```sh
# Install only cmcb-cli package
$ cargo install --git https://github.com/your-username/cat-markdown-code-blocks.git
```

### From Local

For development purposes, clone the project to your local machine, navigate to the root directory of the project, and then run the following command:

```sh
$ cargo install --path ./cli
```

## Usage

```sh
$ cmcb -h
Usage: cmcb [OPTIONS] <FILE_PATHS>...

Arguments:
  <FILE_PATHS>...  Input file or directory paths

Options:
  -c, --clipboard  Copy to clipboard
  -h, --help       Print help
  -V, --version    Print version
```
