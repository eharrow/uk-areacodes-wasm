

  <h1><code>uk-areacodes-wasm</code></h1>

  <strong>Generates a WASM from uk-areacodes-rust.</strong>

![Build](https://github.com/eharrow/uk-areacodes-wasm/actions/workflows/rust.yml/badge.svg)
![Release](https://github.com/eharrow/uk-areacodes-wasm/actions/workflows/release.yml/badge.svg)


## About
Packages up [uk-areacodes-rust](https://github.com/eharrow/uk-areacodes-rust) as a WASM package for use in webassembly runtimes such as a browser.

## 🚴 Usage
### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```
