# Changelog

## 0.3.0 (Upcoming)
- Complete validation of hash inputs.
- Store decoded info in a `Vec<Vec<u8>>` on rust side and convert back on `js` side to reduce the number of calls.
- Error handling with error messages to the `js` client.

## 0.2.0
- Create hash decoding in parts with `rust`.
- Create base HTML page and style.
- Create basic `js` app for UI interactions and `wasm` calls.

## 0.1.0
- Working concept for `rust` -> `WASM` integration.
- Add base HTML template.
