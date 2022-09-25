# wasm-examples

## Reference
[Crate md5](https://docs.rs/md5/latest/md5/index.html)
[Crate sha2](https://docs.rs/sha2/latest/sha2/index.html)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp file_hash_value_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp file_hash_value_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
```
