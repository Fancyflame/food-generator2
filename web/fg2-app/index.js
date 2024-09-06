//import * as wa from "food-generator2-wasm";
let wa = require("food-generator2-wasm");

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

async function foo() {
    let data = await fetchRemoteFile("./cache.fg2");
    data = new Uint8Array(data);
    let lib = wa.Library.load_lib(data);

    let input = prompt("输入文本：");
    if (input === null) {
        return;
    }
    let output = lib.encode(input);
    alert(output);
}
foo()