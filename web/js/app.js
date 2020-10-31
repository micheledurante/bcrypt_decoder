import init, { AlgoType, HashParts, get_algo} from '../wasm/bcrypt_decoder.js';

async function getHashParts(hash) {
    console.log(get_algo(hash));
    console.log(AlgoType[get_algo(hash)]);
}

async function run() {
    await init();

    document.getElementById('decode').onclick = function() {
        try {
            getHashParts(document.getElementsByName('hash')[0].value);
        } catch (e) {
            console.log(e);
        }
    };
}

run();
