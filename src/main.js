const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

let textOutput;

let buttonHeads;
let buttonTails;

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  textOutput = document.querySelector("#txt_output");
  buttonHeads = document.querySelector("#btn_heads");
  buttonTails = document.querySelector("#btn_tails");
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

async function flip(x) {
  var result = await invoke("flip_coin", {guess : x});
  textOutput.textContent = result;
  buttonHeads.disabled = true;
  buttonTails.disabled = true;

  setTimeout( () => {
    textOutput.textContent = "Kopf oder Zahl?"

    buttonHeads.disabled = false;
    buttonTails.disabled = false;
  }, 2000)
}

window.greet = greet;
window.count = count;
window.new_count = new_count;
window.flip = flip;
