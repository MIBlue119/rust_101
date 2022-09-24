#!/bin/bash

# Download the related source 
wget https://raw.githubusercontent.com/sonos/tract/main/examples/onnx-mobilenet-v2/grace_hopper.jpg -O grace_hopper.jpg

# Download the ONNX model
wget https://github.com/onnx/models/raw/main/vision/classification/mobilenet/model/mobilenetv2-7.onnx -O mobilenetv2-7.onnx