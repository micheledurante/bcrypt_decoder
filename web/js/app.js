const js = import("../wasm/bcrypt_decoder.js");
js.then(js => {
  js.greet('Rust with WASM');
});
