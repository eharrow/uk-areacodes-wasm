import * as wasm from "uk-areacodes-wasm";

let marker;
let circle;
const CENTRAL_UK = [53.7974185, -1.5437941];
const map = L.map('map').setView([53.7974185, -1.5437941], 5);
L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
  maxZoom: 19,
  attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
}).addTo(map);

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
  let res = wasm.find_place_by_code(p);

  // cleanup the map before adding markers for a new place
  if (marker) marker.remove();
  if (circle) circle.remove();

  if(res.area !== "") {
    document.getElementById("target").innerHTML = `🎉 ${res.area}`;
    map.setView([res.lat, res.lon], 9);
    marker = L.marker([res.lat, res.lon]).addTo(map);
    circle = L.circle([res.lat, res.lon], {
      color: 'red',
      fillColor: '#f03',
      fillOpacity: 0.5,
      radius: 7000
    }).addTo(map);
  } else {
    document.getElementById("target").innerHTML = "🙁 No match";
    map.setView(CENTRAL_UK, 5);
  }
}
