import * as wa from "food-generator2-wasm";
wa;
/**
 * @type {wa.Library | null}
 */
export let GLOBAL_LIB = null;

export async function load_lib() {
    let data = await fetchRemoteFile("./cache.fg2");
    data = new Uint8Array(data);
    let lib = wa.Library.load_lib(data);

    if (GLOBAL_LIB !== null) {
        GLOBAL_LIB.free();
    }
    GLOBAL_LIB = lib;
}

function fetchRemoteFile(url) {
    return new Promise((resolve, reject) => {
        const xhr = new XMLHttpRequest();

        xhr.open('GET', url, true);
        xhr.responseType = 'arraybuffer';

        xhr.onload = function () {
            if (xhr.status >= 200 && xhr.status < 300) {
                resolve(xhr.response);  // 成功时返回 Blob 数据
            } else {
                reject(`Error: ${xhr.status} ${xhr.statusText}`);
            }
        };

        xhr.onerror = function () {
            reject('Network Error');
        };

        xhr.send();
    });
}