import * as wasm from "uk-areacodes-wasm";

// console.log(wasm.find_code("01582"));
document.querySelector("button").addEventListener("click", () => {
  lookup();
});
// Get the input field
const input = document.getElementById("place");

// Execute a function when the user presses a key on the keyboard
input.addEventListener("keypress", function(event) {
  // If the user presses the "Enter" key on the keyboard
  if (event.key === "Enter") {
    // Cancel the default action, if needed
    event.preventDefault();
    // Trigger the button element with a click
    document.querySelector("button").click();
  }
});
function lookup() {
  const p = input.value.trim();
  let res = wasm.find_code(p);
  document.getElementById("target").innerHTML = res !== "" ? `🎉 ${res}` : "🙁 No match";
}
