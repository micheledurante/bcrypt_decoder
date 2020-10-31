import init, { HashParts, AlgoType, get_hash_parts } from '../wasm/bcrypt_decoder.js';

async function getHashParts(hash) {
    console.log(get_hash_parts(hash) instanceof HashParts);
    console.log(AlgoType[get_hash_parts(hash).algo]);
    console.log(get_hash_parts(hash).cost);
}

async function run() {
    await init();

    document.getElementById('decode').onclick = function() {
        getHashParts(document.getElementsByName('hash')[0].value);
    };
}

run();
