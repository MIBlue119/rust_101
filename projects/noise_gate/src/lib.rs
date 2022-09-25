
// This line staticalling guarantees that we only write Safe Rust code.
//Ref:https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html
#![forbid(unsafe_code)]

// Some linting settings: https://doc.rust-lang.org/rustdoc/lints.html
#![deny(
    // missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    future_incompatible,
    bare_trait_objects,
    elided_lifetimes_in_paths,
    trivial_casts,
    unreachable_pub
)]

// Import required modules.
// A `use` statement declaration creates one or more local name bindings with some other path
// Ref: https://doc.rust-lang.org/reference/items/use-declarations.html
// bring the dasp::sample::SignedSample trait to the local name `SignedSample`.
use dasp::sample::SignedSample; 

// bring the dasp::{Frame, Sample} traits to the local name `Frame` and `Sample`.
use dasp::{Frame, Sample};

// Let compiler provide basic implementations for some traits via the `#[derive()]` attribute.
// it would auto generate some code for the struct.
//  - Clone, to create T from &T via a copy.
//  - Debug, to format a value using the {:?} formatter.
//  - PartialEq, to compare for equality.
// Ref: https://doc.rust-lang.org/rust-by-example/trait/derive.html
// Allows new items to be automatically generated for data structures.
// Ref: https://doc.rust-lang.org/reference/attributes/derive.html
#[derive(Debug, Clone, PartialEq)]
// Rust `struct`: hold multiple related values of different types.
// Ref: https://doc.rust-lang.org/book/ch05-01-defining-structs.html
// use `pub` modifier to make the struct public.
// Ref: https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html
pub struct NoiseGate<S>{
    /// Determines the threshold of the noise gate.
    // The volume level which the noise gate will open.
    pub open_threshold: S,
    // The amount of time(in samples) the noise gate will wait before fully closing.
    pub release_time: usize,
    state: State,
}



fn below_threshold<F>(frame: F, threshold: F::Sample)->bool 
// `where` clause is introduced to allow more expressive bound-checking
// Ref: https://stackoverflow.com/questions/28405621/what-is-the-syntax-and-semantics-of-the-where-keyword
where 
    F: Frame, 
{   
    // dasp::Sample trait provides a method to convert the value to signed
    // Ref: https://docs.rs/dasp/0.11.0/dasp/trait.Sample.html#method.to_signed_sample
    let threshold = abs(threshold.to_signed_sample());
    let negated_threshold = 
        F::Sample::EQUILIBRIUM.to_signed_sample() - threshold;

    frame
        .channels()
        .map(|sample| sample.to_signed_sample())
        .all(|sample| negated_threshold < sample && sample < threshold)
} 


fn abs<S: SignedSample>(sample: S) -> S {
    let zero = S::EQUILIBRIUM;
    if sample >= zero {
        sample
    } else {
        -sample
    }
}
// Use `enum` to define a type which can be one of a few different variants.
// Ref: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    Open, 
    Closing {remaining_samples: usize},
    Closed,
}

// use `impl` to define the implementation block for a type.
// Ref: https://doc.rust-lang.org/std/keyword.impl.html
impl<S> NoiseGate <S> {
    // Define the constuctor 
    pub const fn new(open_threshold:S, release_time: usize)-> Self{
        NoiseGate{
            open_threshold,
            release_time,
            state: State::Closed,
        }
    }
     // Check whether the gat is opened
    pub fn is_open(&self)->bool{
        // use `match` to control flow based on pattern matching.
        // Ref: https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html?highlight=match#match-arms
        match self.state {
            State::Open | State::Closing { .. } => true,
            State::Closed => false
        }
    }
    // Check whether the gat is closed
    pub fn is_closed(&self)-> bool{ !self.is_open()}

}


impl<S: Sample> NoiseGate <S>{
    // Process a btch of frames, passing spans of noise through to a `sink` 
    pub fn  process_frames<K, F>(&mut self, frames: &[F], sink: &mut K)
    where 
        F: Frame<Sample = S>, 
        K: Sink<F>,
    {
        for &frame in frames {
            let previously_open = self.is_open();

            self.state = next_state(
                self.state,
                frame,
                self.open_threshold,
                self.release_time,
            );

            if self.is_open(){
                sink.record(frame);
            }else if previously_open{
                // the gate was previously open, but now it is closed.
                sink.end_of_transmission();
            }
        }
    }
}




fn next_state<F>(
    state: State,
    frame: F, 
    open_threshold: F::Sample,
    release_time: usize,
)-> State
where 
    F: Frame,
{
    match state {
        State::Open => {
            if below_threshold(frame, open_threshold){
                State::Closing{
                    remaining_samples: release_time,
                }
            }else{
                State::Open
            }
        },

        State::Closing{ remaining_samples }=> {
            if below_threshold(frame, open_threshold){
                if remaining_samples == 0 {
                    State::Closed
                } else{
                    State::Closing{
                        remaining_samples: remaining_samples -1,
                    }
                }
            }else{
                State::Open
            }
        },

        State::Closed =>{
            if below_threshold(frame, open_threshold){
                State::Closed
            } else {
                State::Open
            }
        },
    }
}

// A consumer of `Frames` 
pub trait Sink<F>{
    /// Add a frame to the current recording, starting a new recording if necessary.
    fn record(&mut self, frame:F);

    /// Indicate that no more frames will be recorded.
    fn end_of_transmission(&mut self);
}

// `#[cfg(test)]` annotation indicates that the following module is only compiled when run with `cargo test`
// Ref: https://doc.rust-lang.org/book/ch11-03-test-organization.html
#[cfg(test)]
mod tests{

    use super::*;

    const OPEN_THRESHOLD: i16 = 100; 
    const RELEASE_TIME: usize =  5; 

    macro_rules! test_state_transition{
        ($name:ident: $from:expr, $sample:expr => $expected:expr)=>{
            #[test]
            fn $name(){
                let start: State = $from; 
                let expected: State = $expected;
                let frame: [i16; 1] = [$sample];

                let got = 
                    next_state(start, frame, OPEN_THRESHOLD, RELEASE_TIME);
                assert_eq!(got, expected);
            }
        }
    }

    test_state_transition!(open_to_open: State::Open, 101 => State::Open);
    test_state_transition!(open_to_closing: State::Open, 40 => State::Closing {remaining_samples: RELEASE_TIME});    
    test_state_transition!(closing_to_closed: State::Closing { remaining_samples: 0 }, 40 => State::Closed);
    test_state_transition!(closing_to_closing: State::Closing { remaining_samples: 1 }, 40 => State::Closing { remaining_samples: 0 });
    test_state_transition!(reopen_when_closing: State::Closing { remaining_samples: 1 }, 101 => State::Open);
    test_state_transition!(closed_to_closed: State::Closed, 40 => State::Closed);
    test_state_transition!(closed_to_open: State::Closed, 101 => State::Open);

}