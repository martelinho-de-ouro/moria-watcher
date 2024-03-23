# moria-watcher

[![ci](https://github.com/martelinho-de-ouro/moria-watcher/actions/workflows/ci.yml/badge.svg)](https://github.com/martelinho-de-ouro/moria-watcher/actions/workflows/ci.yml)

no one knows the name of this thing.

## This uses the following

### Dependencies

* `clap`
* `assert_cmd`
* `toml`
* `log`
* `async-std`
* `sea-orm`
* `sea-orm-migration`
* `tokio`

### CI

* `actions/checkout@v4`
* `Swatinem/rust-cache@v2`
* `taiki-e/install-action@cargo-llvm-cov`
  * Local development workflow use the report to navigate on html and see uncovered code:

    ```sh
    cargo install cargo-llvm-cov
    cargo llvm-cov --html --open
    ```

* `schneegans/dynamic-badges-action@v1.7.0` to publish the coverage results to a `gist` and use that gist with `shields.io` to provide a coverage badge. (thanks to: <https://bitspittle.dev/blog/2022/kover-badge>)
