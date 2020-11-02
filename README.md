# Bcrypt-Decoder.com
Bcrypt-Decoder.com allows you to decode Bcrypt hashes into their readable parts. You can use it with a generated Bcrypt hash to know which parts it is composed of.

## Build
Build with:
```
wasm-pack build --target web --out-dir web/wasm --release
```

## Tests
Test with:
```
cargo test
```

## How to Release
1. If not present already, create a new section for the `[Upcoming]` release in `CHANGELOG.md`.
1. Replace `[Upcoming]` from the current release with the today's date using the format `[YYYY-MM-DD]`.
1. Bump up the release number in `Cargo.toml`.
