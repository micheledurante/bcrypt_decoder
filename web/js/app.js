import init, {
    AlgoType,
    HashParts,
    get_algo,
    get_cost,
    get_salt,
    get_hash,
    get_salt_bytes,
    get_hash_bytes
} from '../wasm/bcrypt_decoder.js';

let cache;

function isCacheHit(hash) {
    return cache === hash;
}

function hideResultDivs(divs) {
   for (let div of divs) {
        div.style.display = 'none';
   }
}

function showError() {
   document.getElementById('error').style.display = 'block';
}

function setHashParts(hash) {
    document.getElementById('algo').innerHTML = AlgoType[get_algo(hash)];
    const cost = get_cost(hash);
    document.getElementById('cost').innerHTML = cost;
    document.getElementById('iterations').innerHTML = Math.pow(2, cost);
    document.getElementById('salt').innerHTML = get_salt(hash);
    document.getElementById('hash').innerHTML = get_hash(hash);
    document.getElementById('salt-bytes').value = get_salt_bytes(hash);
    document.getElementById('hash-bytes').value = get_hash_bytes(hash);
    document.getElementById('success').style.display = 'block';
}

async function run() {
    await init();

    document.getElementById('decode').onclick = function() {
        if (isCacheHit(document.getElementById('hash-input').value)) {
            return;
        }

        cache = document.getElementById('hash-input').value;

        hideResultDivs(document.getElementsByClassName('result-wrapper'));

        try {
            setHashParts(cache);
        } catch (e) {
            showError();
        }
    };
}

run();
