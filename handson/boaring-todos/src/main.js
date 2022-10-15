const { invoke } = window.__TAURI__.tauri;
import { listen } from "@tauri-apps/api/event";

let xInputEl;
let yInputEl;
let resultEl;
let clickMeEl;
let businessValueEl;
let eventInfoEl;

window.addEventListener("DOMContentLoaded", () => {
  xInputEl = document.querySelector("#x-value");
  yInputEl = document.querySelector("#y-value");
  resultEl = document.querySelector("#result");
  businessValueEl = document.querySelector("#business-value");
  eventInfoEl = document.querySelector("#event-info");

  clickMeEl = document.querySelector("#click-me");
  clickMeEl.addEventListener("pointerup", async function () {
    let quote = await invoke("get_random_quote");
    clickMeEl.textContent = quote;
    setTimeout(function () {
      clickMeEl.textContent = "Tekrar bas";
    }, 2000)
  });

  listen("menu-event", (e) => {
    eventInfoEl.textContent = "Backend taraftan " + e.payload + " olayı tetiklendi";
  })
});

async function sum() {
  resultEl.textContent = await invoke("sum_of_two", { x: xInputEl.value, y: yInputEl.value });
}

async function increase(){
  let bv = await invoke("increase_value",{v: 1});
  businessValueEl.textContent = bv;
}

async function decrease(){
  let bv = await invoke("decrease_value",{v: 1});
  businessValueEl.textContent = bv;
}

window.sum = sum;
window.increase = increase;
window.decrease = decrease;
