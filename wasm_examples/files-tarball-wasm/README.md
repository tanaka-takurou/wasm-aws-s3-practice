# wasm-examples

## Reference
[Crate serde](https://docs.rs/serde/latest/serde/index.html)
[Crate tar](https://docs.rs/tar/latest/tar/index.html)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp files_tarball_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp files_tarball_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
```
