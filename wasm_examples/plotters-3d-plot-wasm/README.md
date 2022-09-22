# wasm-examples

## Reference
[Crate plotters](https://docs.rs/plotters/latest/plotters/index.html)
[plotters/examples/3d-plot2](https://github.com/plotters-rs/plotters/blob/master/plotters/examples/3d-plot2.rs)

## Build
```
wasm-pack build --target web
```

## Upload
```
aws s3 cp index.html s3://bucket-name --content-type "text/html"
aws s3 cp plotters_3d_plot_wasm.js s3://bucket-name --content-type "text/javascript"
aws s3 cp plotters_3d_plot_wasm_bg.wasm s3://bucket-name --content-type "application/wasm"
```
