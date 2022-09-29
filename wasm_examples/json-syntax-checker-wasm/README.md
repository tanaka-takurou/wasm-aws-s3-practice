# wasm-examples

## Reference
[Crate js_sys](https://rustwasm.github.io/wasm-bindgen/api/js_sys/index.html)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp json_syntax_checker_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp json_syntax_checker_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
```
