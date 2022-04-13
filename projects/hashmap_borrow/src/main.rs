// It it an example to calculate the word frequency by HashMap
use std::collections::HashMap;

fn main(){
    let text = "hello world hello";

    //Construct a new HashMap
    let mut freqs = HashMap::new();

    for word in text.split_whitespace(){

        //Use Entry method to replace
        *freqs.entry(word).or_insert(0) +=1;

        // match freqs.get_mut(word){
        //     Some(value) => *value +=1,
        //     None =>{
        //         freqs.insert(word,1);
        //     },
        // }
    }
    // {:#?} annotation could print standard structure
    println!("Word frequencies: {:#?}", freqs);
}
