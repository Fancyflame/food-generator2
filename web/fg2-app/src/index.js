import { load_lib, GLOBAL_LIB } from "./lib_mgr.js";

load_lib("./cache.fg2").then(() => {
    console.log("库加载完毕");
});

const inputArea = document.getElementById("input");
const outputArea = document.getElementById("output");
const encBtn = document.getElementById("enc-btn");
const decBtn = document.getElementById("dec-btn");

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