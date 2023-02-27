# template-rust-aws-lambda
template-rust-aws-lambda

This is a template to be used as boilerplate to start developing AWS Lambdas in Rust and compile for target `aarch64-unknown-linux-gnu` AKA: arm64, Graviton2.

It is meant to be used with https://github.com/cargo-generate/cargo-generate


```bash
cargo install cargo-generate
```


```bash
cargo generate --git https://github.com/954alberto/template-rust-aws-lambda.git
🤷   Project Name: rust-lambda-test
🔧   Destination: /Users/<username>/Projects/rust-lambda-test ...
🔧   project-name: rust-lambda-test ...
🔧   Generating template ...
[ 1/13]   Done: .cargo/config.toml
[ 2/13]   Done: .cargo
[ 3/13]   Done: .devcontainer/devcontainer.json
[ 4/13]   Done: .devcontainer
[ 5/13]   Done: .github/workflows/aarch64-unknown-linux-gnu.yml                                                                               [ 6/13]   Done: .github/workflows                                                                                                             [ 7/13]   Done: .github                                                                                                                       [ 8/13]   Done: .gitignore                                                                                                                    [ 9/13]   Done: Cargo.lock                                                                                                                    [10/13]   Done: Cargo.toml                                                                                                                    [11/13]   Done: README.md                                                                                                                     [12/13]   Done: src/main.rs                                                                                                                   [13/13]   Done: src                                                                                                                           🔧   Moving generated files into: `/Users/<username>/Projects/rust-lambda-test`...
💡   Initializing a fresh Git repository
✨   Done! New project created /Users/<username>/Projects/rust-lambda-test

```