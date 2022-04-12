//Example to slice arrays, vectors, String


fn main() {
    // Slice string
    let s = String::from("Harry Potter!");
    let s_slice = &s[5..];
    println!("Slice of s {}",s_slice);

    // Slice array
    let a = [0.0, 3.14, -8.78787,10.0, 344.0,];
    let a_slice= &a[2..4];
    println!("Slice of a {:#?}", a_slice);

    //Slice vector 
    let mut v = vec![10, 20, 30];
    v.push(40);
    let v_slice = &v[1..2];
    println!("Slice of v {:#?}", v_slice);

}
