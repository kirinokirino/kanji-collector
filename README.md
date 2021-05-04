<div align="center">

  <h1><code>kanji-collector</code></h1>
  
  wasm function that parses utf-8 string and outputs js object containing kanjis. Kanjis that were together are in the same string. 
  Nothing smart is going on, so different words that didn't have anything separating them will still be in the same string.
  `import * as wasm from "kanji-collector";`
  ```wasm.search_kanji(
        "testこの建物は現代的に見える。君が知ってる人の中で誰が一番賢い？"
    )```
  ###git push --set-upstream origin master

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

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

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
