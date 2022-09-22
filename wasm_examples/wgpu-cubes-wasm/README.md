# wasm-examples

## Reference
[Crate wgpu](https://docs.rs/wgpu/latest/wgpu/)
[learn-wgpu/beginner/tutorial9-models](https://sotrh.github.io/learn-wgpu/beginner/tutorial9-models/)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp wgpu_cubes_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp wgpu_cubes_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
aws s3 cp cube.zip s3://bucket-name/res/cube.zip --content-type "application/zip"
aws s3 cp cube.obj s3://bucket-name/res/cube.obj --content-type "application/octet-stream"
aws s3 cp cube.mtl s3://bucket-name/res/cube.mtl --content-type "application/octet-stream"
aws s3 cp cube-normal.png s3://bucket-name/res/cube-normal.png --content-type "image/png"
aws s3 cp cube-diffuse.jpg s3://bucket-name/res/cube-diffuse.jpg --content-type "image/jpeg"
```
