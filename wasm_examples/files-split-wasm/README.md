# wasm-examples

## Reference
[Crate serde](https://docs.rs/serde/latest/serde/index.html)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp files_split_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp files_split_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
```
