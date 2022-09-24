//Src from: https://github.com/sonos/tract/blob/main/examples/onnx-mobilenet-v2/src/main.rs
use tract_onnx::prelude::*;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path
};

fn labels_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>>{
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() -> TractResult<()> {
    // Initial and load the model
    let model = tract_onnx::onnx()
        // Load the model from path    
        .model_for_path("mobilenetv2-7.onnx")?
        // Specify input type and shape
        .with_input_fact(0, f32::fact(&[1,3,224,224]).into())?
        // Optimize the model
        //.into_optimized()?
        // Make the model runnable and return it
        .into_runnable()?;

    // Open image net labels file
    let labels = labels_from_file("imagenet_slim_labels.txt")?;

    // Open image, resize it and make a Tensor out of it
    let image = image::open("grace_hopper.jpg").unwrap().to_rgb8();
    let resized = 
        image::imageops::resize(&image, 224, 224, ::image::imageops::FilterType::Triangle);
    
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1,3,224,224), | (_, c, y, x) | {
        let mean = [0.485, 0.456, 0.406][c];
        let std = [0.229, 0.224, 0.225][c];
        (resized[(x as _,y as _)][c] as f32 /255.0 - mean ) /std
    })
    .into();

    // Run the model on the input  
    let result = model.run(tvec!(image))?;

    // Find and display the max value with its index
    let best = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(2..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("result: {:?}", best);
    println!("label: {:?}", labels[best.unwrap().1-1]);
    Ok(())
}
