import init, { AlgoType, HashParts, get_algo, get_cost, get_salt, get_hash } from '../wasm/bcrypt_decoder.js';

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

function clearInput() {
    document.getElementById('hash-input').value = '';
}

function showResult() {
    document.getElementById('success').style.display = 'block';
}

function setValues(hash) {
    document.getElementById('algo').innerHTML = AlgoType[get_algo(hash)];
    const cost = get_cost(hash);
    document.getElementById('cost').innerHTML = cost;
    document.getElementById('iterations').innerHTML = Math.pow(2, cost);
    document.getElementById('salt').innerHTML = get_salt(hash);
    document.getElementById('hash').innerHTML = get_hash(hash);
}

function getHashParts() {
    if (isCacheHit(document.getElementById('hash-input').value)) {
        return;
    }

    cache = document.getElementById('hash-input').value;

    hideResultDivs(document.getElementsByClassName('result-wrapper'));

    try {
        setValues(cache);
        showResult();
    } catch (e) {
        showError();
    }
}

document.getElementById('decode').onclick = function() {
    getHashParts();
};

document.getElementById('clear-button').onclick = function() {
    clearInput();
};

(async function run() {
    await init();
})();
