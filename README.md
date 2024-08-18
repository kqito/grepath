# grepath

**grepath** is a command-line utility to effortlessly extract file paths from command outputs. It's especially useful in pipelines for processing compiler or lint tool outputs.

## Features

- Extract file paths from a given text.
- Optionally extract full paths including line and column numbers.
- Remove duplicate paths.

## Installation

TODO

## Usage

You can use `grepath` in command-line pipelines to filter paths from the output of other commands. Here is an example usage:

```sh
bun tsc --noEmit | grepath | xargs -o nvim -p
```

### Command Line Options

- **file**: Optionally specify a file to read from instead of stdin.
- **-d, --debug**: Print debug information.
- **-l, --lines**: Extract paths with line numbers.
- **-u, --unique**: Remove duplicate paths.

## Example

Given a TypeScript compiler error output, you can extract file paths for further processing:

```sh
❯ bun tsc --noEmit | grepath

src/domains/commu/components/TestCard.tsx
src/domains/commu/components/TestCardContainer.tsx
src/domains/commu/components/TestListCard.tsx
```

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/kqito/grepath/issues) if you want to contribute.

## License

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
