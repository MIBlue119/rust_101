//Ownership and borrowing

fn say(s:String){
    println!("I say {}!",s);
}

fn main() {
    let mut a = String::from("hello");
    a.push_str(" Weiren!");

    println!("I say, {}!",a);

    //Move the ownership of the String from a to t
    let t=a;

    say(t);
    // Ownership has been transfered to the function `say`, we couldn't use it again.
    //println!("I say, {}!",t);

    // This  
    //println!("I say, {}!",a);

    // If we want to keep the ownership, we coulde use `clone`
    let mut m = String::from("Mcdownloads");
    say(m.clone());
    println!("Recall {} again",m);

}
