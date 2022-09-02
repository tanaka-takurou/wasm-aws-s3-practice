# wasm-examples

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp sample.wasm s3://bucket-name --content-type "application/wasm"
```
