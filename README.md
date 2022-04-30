# image-cmp-tool

Just a simple tool to compare a rendered image (test image) and corresponding ground-truth image (reference image).

```
USAGE:
    image-cmp-tool.exe [FLAGS] --reference <reference> --test <test>

FLAGS:
        --flip       FLIP (FLIP: A Tool for Visualizing and Communicating Errors in Rendered Images)
    -h, --help       Prints help information
        --mae        MAE (mean absolute error)
        --mse        MSE (mean squared error)
        --psnr       PSNR (peak signal to noise ratio)
        --relmse     relMSE (relative mean squared error)
        --rmae       RMAE (relative mean absolute error)
        --smape      SMAPE (symmetric mean absolute percentage error)
    -V, --version    Prints version information

OPTIONS:
    -r, --reference <reference>    path to the reference image (ground-truth image)
    -t, --test <test>              path to the test image (rendered image)
```

ꟻLIP error is calculated using executable built from [NVlabs/flip](https://github.com/NVlabs/flip) and ꟻLIP mean is taken from the output using regex. Path to ꟻLIP executable can be changed in `src/flip/rs`.

Calculation of other errors is parallelized by [rayon](https://github.com/rayon-rs/rayon).
