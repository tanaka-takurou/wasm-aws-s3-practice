# wasm-examples

## Reference
[Crate bytes](https://docs.rs/bytes/latest/bytes/index.html)
[Crate mp4](https://docs.rs/mp4/latest/mp4/index.html)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp file_to_mp4_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp file_to_mp4_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
```
