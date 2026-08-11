# rust-cli-template

A GitHub template for a simple Rust CLI.

## Use

Create a repository from this template on GitHub, clone it, then run:

```bash
./init.sh mytool "One line description of the tool."
```

An optional third argument sets the GitHub owner (default `eljpsm`).

The script renames the placeholder crate (`renameme`).

```bash
git add -A && git commit -m "Initialize from rust-cli-template"
nix build
```

## What you get

- A nix flake as the only toolchain. `nix develop` for the shell, `nix build`
  for the package. No rustup, no rust-toolchain.toml.
- CI that runs fmt and clippy (`check`) and builds the flake package (`build`),
  which runs the tests in its check phase.
- A binary crate skeleton: `main.rs` dispatches, `cli.rs` holds the clap types,
  `app.rs` is the only module that prints or picks exit codes.
- End-to-end tests against the real binary with a scratch directory sandbox.
- Renovate wired to the shared `eljpsm/renovate-config` preset.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
