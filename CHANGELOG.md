# Changelog

## 0.4.0 [Unreleased]
#### Added
- Create webpage style.
- Store decoded info in a `Vec<Vec<u8>>` on rust side and convert back on `js` side to reduce the number of calls.

#### Removed
- base64 decoding.

## 0.3.0 [2020-11-02]
#### Added
- Complete validation of hash inputs.
- Error handling with error messages to the `js` client.
- Add cache to `js` for last hash. Block request for cache hits.

## 0.2.0 [2020-11-02]
#### Added
- Create hash decoding in parts with `rust`.
- Create base HTML page and style.
- Create basic `js` app for UI interactions and `wasm` calls.

## 0.1.0 [2020-10-31]
#### Added
- Working concept for `rust` -> `WASM` integration.
- Add base HTML template.
