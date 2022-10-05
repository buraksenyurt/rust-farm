const { invoke } = window.__TAURI__.tauri;

let xInputEl;
let yInputEl;
let resultEl;

window.addEventListener("DOMContentLoaded", () => {
  xInputEl = document.querySelector("#x-value");
  yInputEl = document.querySelector("#y-value");
  resultEl = document.querySelector("#result");
});

async function sum() {  
  resultEl.textContent = await invoke("sum_of_two", { x: xInputEl.value,y: yInputEl.value });
}

window.sum = sum;
