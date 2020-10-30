import init, { greet } from '../wasm/bcrypt_decoder.js';

async function decode(params) {
    greet(params);
}

async function run() {
    await init();

    document.getElementById('decode').onclick = function() {
        decode(document.getElementsByName('hash')[0].value);
    };
}

run();
