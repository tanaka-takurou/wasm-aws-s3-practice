# wasm-examples

## Reference
[Crate image](https://docs.rs/image/latest/image/index.html)
[Crate num-complex](https://docs.rs/num-complex/latest/num-complex/index.html)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp file_to_png_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp file_to_png_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
```
