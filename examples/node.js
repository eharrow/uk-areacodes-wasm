import pkg from '../pkg/uk_areacodes_wasm.js';
const { find_code } = pkg;

console.log(find_code("01553"));
console.log(`01582 is the code for ${find_code('01582')}`);


