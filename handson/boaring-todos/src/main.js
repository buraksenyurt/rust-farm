const { invoke } = window.__TAURI__.tauri;

let xInputEl;
let yInputEl;
let resultEl;
let clickMeEl;

window.addEventListener("DOMContentLoaded", () => {
  xInputEl = document.querySelector("#x-value");
  yInputEl = document.querySelector("#y-value");
  resultEl = document.querySelector("#result");

  clickMeEl = document.querySelector("#click-me");
  clickMeEl.addEventListener("pointerup", async function () {
    let quote = await invoke("get_random_quote");
    clickMeEl.textContent = quote;
    setTimeout(function () {
      clickMeEl.textContent = "Tekrar bas";
    }, 2000)
  })
});

async function sum() {
  resultEl.textContent = await invoke("sum_of_two", { x: xInputEl.value, y: yInputEl.value });
}

window.sum = sum;
