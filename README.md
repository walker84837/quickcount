# QuickCount

> Sleek, blazingly fast and minimalist word counter for the desktop.

[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE.md)

## Table of Contents

- [Usage](#usage)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [License](#license)

## Usage

You can build this repo from source (you need [the Rust toolchain](https://rustup.rs)):

1. Clone the repository:
    ```console
    git clone https://github.com/walker84837/quickcount.git
    ```
2. It may take a while at first, depending on where you're compiling, run:
    ```console
    cargo build --release
    ```
3. If you prefer, you can move the executable file to some other place:
    ```console
    mv target/release/quickcount path/to/your/destination
    ```
4. If prefer to keep the binary in the same directory, you can run the code with Cargo (you can do this without doing step 2 as it builds automatically):
    ```console
    cargo run --release
    ```

## Contributing

Contributions are always welcome!
If you encounter problems or have questions, feel free to open an issue.

## Roadmap

- [ ] Add support for selecting text (and consequently copy/paste it)
- [ ] Add support for clicking on text

## License

This project is released under the [BSD-3-Clause](LICENSE.md) license.
