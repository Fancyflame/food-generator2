import { load_lib, GLOBAL_LIB } from "./lib_mgr.js";

load_lib("./cache.fg2").then(() => {
    console.log("库加载完毕");
});

const [inputArea, outputArea, encBtn, decBtn, pasteBtn, copyBtn] = [
    "input", "output", "enc-btn", "dec-btn", "paste", "copy"
].map(id => document.getElementById(id));

encBtn.onclick = () => {
    let out = GLOBAL_LIB.encode(inputArea.value);
    outputArea.style.color = "";
    outputArea.value = out;
};

decBtn.onclick = () => {
    try {
        outputArea.value = GLOBAL_LIB.decode(inputArea.value);
        outputArea.style.color = "";
    } catch (err) {
        outputArea.value = err;
        outputArea.style.color = "orangered";
    }
};

pasteBtn.onclick = async function () {
    let txt = await navigator.clipboard.readText();
    inputArea.value = txt;
}

copyBtn.onclick = async function () {
    let txt = outputArea.value;
    await navigator.clipboard.writeText(txt);
    copyBtn.innerText = "复制成功";
    await waitTimeout(500);
    copyBtn.innerText = "复制";
}

function waitTimeout(ms) {
    return new Promise((resolve, _) => {
        setTimeout(resolve, ms);
    });
}