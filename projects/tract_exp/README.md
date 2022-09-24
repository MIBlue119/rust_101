# Tract Exp

- [Tract](https://github.com/sonos/tract): Tiny, no-nonsense, self-contained, Tensorflow and ONNX inference written on Rust from Sonos

- This is a basic example to inference a mobilenet classification model at a image
    - original code from: https://github.com/sonos/tract/tree/main/examples/onnx-mobilenet-v2
    - revise some part to load label file

## Usage

1. Download realated src `mobilenetv2-7.onnx` / `grace_hopper.jpg` 
```
$./download_related_src.sh
```

2. Compile and run the code
```
$ cargo run 
```

3. We could get the result 
```
result: Some((11.475581, 654))
label: "military uniform"
```