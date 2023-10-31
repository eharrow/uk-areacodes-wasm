import * as wasm from "uk-areacodes-wasm";

// console.log(wasm.find_code("01582"));
document.querySelector("button").addEventListener("click", () => {
  lookup();
});
function lookup() {
  const p = document.getElementById("place").value;
  let res = wasm.find_code(p);
  document.getElementById("target").innerHTML = res !== "" ? `✅ ${res}` : "❌ No match";
}
