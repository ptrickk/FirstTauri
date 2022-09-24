const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

let textOutput;

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  textOutput = document.querySelector("#txt_output");
});

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function count() {
  textOutput.textContent = await invoke("count", { num: "1"});
}

async function new_count() {
  var temp = await invoke("add_count", {num : 3});
  textOutput.textContent = temp;
}

window.greet = greet;
window.count = count;
window.new_count = new_count;
