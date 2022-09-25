# wasm-examples

## Reference
[Crate std](https://doc.rust-lang.org/std/primitive.u64.html)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp file_data_convert_be_le_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp file_data_convert_be_le_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
```
