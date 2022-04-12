
struct Person{
    name: String,
}
// We add the `ampersand` == `&` before the type to show we borrow the owner ship of the argument
fn congratulate(person: &Person){ 
    println!("Congratulations, {}!!!", person.name);
}

// This would cause error, because the n would be cleaned after the function scope
// So we couldn't use `&`  reference to return a value.
// fn name() -> &str {
//     let n =String::from("Weiren");
//     &n 
// }
// We may need to change to return Value

fn name()-> String{
    let n = String::from("Weiren");
    n
}

fn main() {
    //Initiate an instance
    let p = Person{
        name: String::from("Weiren"),
    };

    //Borrow the ownership of the instance
    congratulate(&p);
    println!("We could still use p {} here", p.name);

    println!("{}",name());
}
