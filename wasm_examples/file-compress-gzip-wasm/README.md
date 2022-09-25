# wasm-examples

## Reference
[Crate flate2](https://docs.rs/flate2/latest/flate2/index.html)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp file_compress_gzip_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp file_compress_gzip_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
```
