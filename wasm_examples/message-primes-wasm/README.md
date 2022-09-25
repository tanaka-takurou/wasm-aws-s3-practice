# wasm-examples

## Reference
[Crate primes](https://docs.rs/primes/latest/primes/index.html)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp message_primes_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp message_primes_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
```
