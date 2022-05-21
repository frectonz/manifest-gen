# manifest-gen

`manifest-gen` is a CLI tool that helps you in creating a `manifest.json` file for your pwa.

## Demo

[![asciicast](https://asciinema.org/a/496160.svg)](https://asciinema.org/a/496160)

## Installation

```sh
cargo install manifest-gen
```

## Usage

```sh
manifest-gen icon.png
```

`icon.png` is the image of your pwa.

### What does it do?

- Resizes the given image to appropriate sizes
- Asks a bunch of questions and fills in the fields in the manifest.json
