import init, { AlgoType, HashParts, get_algo, get_cost, get_salt, get_hash } from '../wasm/bcrypt_decoder.js';

async function getHashParts(hash) {
    console.log(AlgoType[get_algo(hash)]);
    console.log(get_cost(hash));
    console.log(get_salt(hash));
    console.log(get_hash(hash));
}

async function run() {
    await init();

    document.getElementById('decode').onclick = function() {
        getHashParts(document.getElementsByName('hash')[0].value);
    };
}

run();
