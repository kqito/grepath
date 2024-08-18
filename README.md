# grepath

**grepath** is a command-line utility to effortlessly extract file paths from command outputs. It's especially useful in pipelines for processing compiler or lint tool outputs.

## Features

- Extract file paths from a given text.
- Optionally extract full paths including line and column numbers.
- Remove duplicate paths.

## Installation

### Install prebuilt binaries via Homebrew

```sh
brew install kqito/tap/grepath
```

### Install prebuilt binaries via shell script

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/kqito/grepath/releases/latest/download/grepath-installer.sh | sh
```

Or download directly from [releases](https://github.com/kqito/grepath/releases).

## Usage
When we run `grepath <file>`, it reads the file and outputs the paths from the text.

For example, if you want to open error files in VSCode, you can use the following command:

```sh
grepath error.log | xargs -o code
```


### Pipe from a command
You can use `grepath` in command-line pipelines to filter paths from the output of other commands. Here is an example usage:

```sh
cat error.log | grepath | xargs -o code
```

### Command Line Options

```sh
grepath --help
Usage: grepath [<file>] [-d] [-l] [-u]

Args

Positional Arguments:
  file              file

Options:
  -d, --debug       help
  -u, --unique      unique Omit duplicate paths
  --help            display usage information
```

## TODO
- [ ] Support for windows paths
- [ ] Support for custom regex patterns


## Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/kqito/grepath/issues) if you want to contribute.

## License

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
