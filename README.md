# BcryptDecoder
Bcrypt-Decoder.com allows you to decode Bcrypt hashes into their readable parts. You can use it with a generated Bcrypt hash to know which parts it is composed of.

## Installation
1. Install `rustup`
1. Install wasm `cargo install wasm-pack`
1. Run `cargo build`
1. Run `cargo test`

## Build
```
wasm-pack build --target web --out-dir web/wasm --release
```

## Test
```
cargo test
```

## How to Release
1. If not present already, create a new section for the `[Upcoming]` release in `CHANGELOG.md`.
1. Replace `[Upcoming]` from the current release with the today's date using the format `[YYYY-MM-DD]`.
1. Bump up the release number in `Cargo.toml`.
