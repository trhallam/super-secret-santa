# Super Secret Santa

This project is heavily inspired by [secretsanta](https://github.com/arcanis/secretsanta/tree/gh-pages). However, I wanted
to challenge myself to write as must of the text processing as possible in Rust, which then via WASM would provide the
functionality for the app. The Typescript portion of the application is mainly for styling and interaction with the user.

## Build

```shell
wasm-pack build
```

## Tests

```shell
wasm-pack test --node
```

```shell
cargo test
```