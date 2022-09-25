# wasm-examples

## Reference
[Crate base64](https://docs.rs/base64/latest/base64/index.html)
[Crate image](https://docs.rs/image/latest/image/index.html)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp img_converter_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp img_converter_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
```
