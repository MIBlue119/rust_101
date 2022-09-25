# Noise Gate 


- Reference: https://adventures.michaelfbryan.com/posts/audio-processing-for-dummies/
    - github: https://github.com/Michael-F-Bryan/noise-gate
- Try to use rust to implement `noise gate`
    - split the audio to clips based on volume 

## Example to use

```console
$ cargo run --release --example wav-splitter -- \
    --output-dir output \
    --threshold 50 \
    --release-time 0.3 \
    data/KBDL-B17-Tribute-20191005.wav
$ ls output
clip_0.wav   clip_3.wav   clip_6.wav   clip_9.wav   clip_12.wav  clip_15.wav
clip_18.wav  clip_21.wav  clip_1.wav   clip_4.wav   clip_7.wav   clip_10.wav
clip_13.wav  clip_16.wav  clip_19.wav  clip_22.wav  clip_2.wav   clip_5.wav
clip_8.wav   clip_11.wav  clip_14.wav  clip_17.wav  clip_20.wav
```


## Sources
- [dasp](https://crates.io/crates/dasp): A crate providing the fundamentals for working with audio PCM DSP
- [hound](https://docs.rs/hound/latest/hound/): A wav encoding and decoding library