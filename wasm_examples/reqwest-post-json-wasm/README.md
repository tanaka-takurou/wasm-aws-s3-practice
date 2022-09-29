# wasm-examples

## Reference
[Crate reqwest](https://docs.rs/reqwest/latest/reqwest/)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp reqwest_post_json_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp reqwest_post_json_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
```
