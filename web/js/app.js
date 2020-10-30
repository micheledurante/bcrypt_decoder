import init, { greet } from '../wasm/bcrypt_decoder.js';

async function run() {
    await init();
    greet('Rust with WASM');
}

run();
