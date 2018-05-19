### Install

    curl https://sh.rustup.rs -sSf | sh


### [Additional packages](http://crates.io/)

[`ripgrep`](https://crates.io/crates/ripgrep):  Line oriented search tool using Rust's regex library. Combines the raw performance of grep with the usability of the silver searcher.

    cargo install ripgrep

[`exa`](https://crates.io/crates/exa):  A modern replacement for `ls`
**:warning: Requirements:** `apt install cmake`

    cargo install exa

[`wkdr`](https://crates.io/crates/wkdr):  A cli tool to access Wikidata

**:warning: Requirements:** `apt install libssl-dev`

    cargo install wkdr

:information_source:

### Init Rust project

    cd rust-workshop
    cargo init

Edit `Cargo.toml` to add:

    [dependencies]
    clap = "2.31.2"

Install dependencies

    cargo update

Run project:

    cargo run
