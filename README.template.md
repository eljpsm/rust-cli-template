# renameme

A placeholder command line tool.

## Install

```bash
# With cargo:
cargo install --git https://github.com/eljpsm/renameme

# With nix:
nix run github:eljpsm/renameme

# Or from a checkout:
make install
```

Prebuilt Linux binaries are on the [releases
page](https://github.com/eljpsm/renameme/releases).

## Usage

```bash
# Print a greeting.
renameme greet
renameme greet --shout crab

# Count the lines in a file.
renameme count notes.txt
```

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
