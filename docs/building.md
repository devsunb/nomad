# Building from Source

Nomad can be built using either Cargo or Nix. Both methods produce a `lua`
folder that you can place anywhere in your Neovim [`runtimepath`][runtimepath].

## Building with Cargo

To build Nomad with Cargo you will need a nightly version of the Rust compiler.
With that installed, you can use the `xtask` command to build the plugin by
doing:

```bash
cargo xtask neovim build
```

This will build a debug version of Nomad for the latest stable Neovim release.
To build a release version, use the `--release` flag. To build for Neovim
Nightly, use the `--nightly` flag. To see all the available build options:

```bash
cargo xtask neovim build --help
```

## Building with Nix

The Nomad flake exposes four packages that you can use to build the plugin with
Nix, following the format `neovim{-nightly}{-debug}`. For example, to build a
release version of Nomad for stable Neovim, you would use:

```bash
nix build .#neovim
```

whereas to build a debug version for Neovim Nightly, you would use:

```bash
nix build .#neovim-nightly-debug
```

All four packages will create the `lua` folder under `result/lua` (relative to
the root of the repo).

[runtimepath]: https://neovim.io/doc/user/options.html#'runtimepath'
