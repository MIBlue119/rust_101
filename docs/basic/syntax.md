


- array size is fixed. we couldn't add new elemnt. 
    - If we want to add new element, we need to use `Vec`

```
fn name(param1: type1, ...) -> return_type{
    ...body...
}
```

- Enums: enumerations, are great for defining types that can be one of a possible set of values.
    - work with `match` expressions
    ```
    enum HockeyPosition {
        Center,
        Wing,
        Defense
    }

    enum Clock{
        Sundial(u8),
        Digital(u8, u8),
        Analog(u8,u8,u8)
    } 

    fn tell_time(clock: Clock){
        match clock {
            Clock::Sundial(hours)=>
                println!("It is about {} o'clock", hours),
            Clock::Analog(hours, minutes, seconds)=>{
                println!("It is {} minutes and {} seconds, {} o'clock", minutes, seconds, hours);
            },
            Clock::Digital(hours, minutes)=>
                println!("It is {} minutes past {}", minutes, hours)
        }
    }

    fn main(){
        tell_time(Clock::Analog(9,24,45));
    }
    ```
- When to use enum vs struct
    - enum
        - Choice between a set of values 
    - struct 
        - Same attribute for each value of a type 
- Structs, short for structures, are custom tyoes that group related data together. 
    ``` enum HockeyPositionP{
        ....
        }

        //struct name with Camel Case
        struct HockeyPlayer {
            number: u8,   // field name with snack case
            position: HockeyPosition,
            goals_ytd: u8
        }

    ```
    - tuple struct
        ```
        struct Triangle(u32, u32,u32);
        ```
    - unit struct : struct without field.
        ```
        struct Pet;
        ```
- method
    ```
    strucht struct_name{
        pet: String
    }
    impl struct_name{
        fn method_name(self, arguments){

        }
    }
    ```
    - Associated funcitons
        - Defined within a `impl` block
        - Don't take self as a parameter 
        - Commonly used to create instances
    - method that write data? how could we do that. 
    ```
    &mut self // for write the self data
    &self // for reading the self data
    ```
### Ownership and borrowing 
- Definiton of Ownership
    - Responsible for cleaning up, or deallocating that chunk of memory.
    - This cleanup happens automatically when the owner variable goes out of scope.
    - The owner of data is also who is responsible for deciding whether that data is mutable or not.
- How ownership differs from?
    - In C, manual memory management with `malloc` and `free`
        - "Use after free": part of programs calls `free` and another parts tries to use that memory again
        - "Memory Leak": forgot to call free that cause space ran out.
        - "Double free": two parts of program try to clean up the same data, that cause memory corruption.
    - In Ruby, with Garbage Collector
        - Keep track of which memory is still in use, and cleans up any data that your program is done with.
        - Downside: lose control and performance
    - Rust is trying to get best of both.
        - Ownership gives you control over memory allocation and associated performance, but by cleaning up data automatically when the owner goes out of `scope`
- The `String` datatype
- Cloning data 
    - We could use `xxx.clone()` to clone the data and keep the ownerships

- Borrowing 
    - What is borrowing? Lend out a value instead of transferring ownership.
        - A way to allow some code to use a value without moving ownership. It's like letting someone borrow one of your toys for a little while.
        - We add the `ampersand` == `&` before the type to show we borrow the owner ship of the argument
        - Example:
            ```
                struct Person{
                    name: String,
                }
                // We add the `ampersand` == `&` before the type to show we borrow the owner ship of the argument
                fn congratulate(person: &Person){
                    println!("Congratulations, {}!!!", person.name);
                }

                fn main() {
                    //Initiate an instance
                    let p = Person{
                        name: String::from("Weiren"),
                    };

                    //Borrow the ownership of the instance
                    congratulate(&p);
                    println!("We could still use p {} here", p.name);
                }
            ```
    - Why borrow? Reduce allocation , improve `performance`
        - If a function doesn't need ownership of a value that has allocated memory, instead of cloning the value and giving to the function.
        - We could give a function the reference(address to the value) to the original value.
            - No extra allocation is needed.
            - No wasting time to cloning the large instance 
        - A signal of intent 
            - when we look at a function signature and see that it borrows its paramters,we may know
                it wants nothing to do with the allocation or deallocation of that data.
    - Difference between borrowing and pointer in other languages(C,C++)? 
        - In rust, the borrow checker ensures at compile time and won't let you try to run code with this problem.
            - You'll never have an invalid reference that point to `nothing` or to `invalid memory`
            - Example: these bugs is very hard to track down at run time.
                - C: segmentation fault 
                - Ruby: undefined method on nil
                - Javascript: undefined is not a function

- Slices
    - What is a slice? A data type that always `borrows` data owned by some other data structure. Borrow a chunk of data.
        - A slice consists of a `pointer` and a `length` 
            - The `pointer` is a reference to the start of the data that slice contains
            - The `length` is the number of elements after the start that the slice contains
    - How to create a slice from Strings, Vecs, or arrays?
        - Example
        ```
            let a = String::from("Taiwan");
            //Extract the position 0~4 
            let a_slice = &a[1..4];
        ```
        - make a slice borrows all of the data: two dots `..` at square brackets. 
        - The borrow checker ensures slices are always valid. At runtime, Rust will panic and stop your program if slice indices are out of bounds.
        - Rust can't prove the slice indices are valid at compile time.
            - It checks the slice indices at runtime and deliberately terminated the program if they're invald.

    - String slices have additional protection 
        - The indices of the slice range must be at valid `Unicode` character boundaries
            - Example: Have a string literal containing emogi and attempt to create slice from 0 to 1. this also compiles. 
               When we run the program, it panics with the error message `byte index 1 is not a char boundry` 
                - `thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside '😀' (bytes 0..4) of `😀👹🫁🙇`', src/main.rs:3:20`
               ```
                let s ="😀👹🫁🙇";
                let s_slice = &s[0:1];
                println!("s_slice is {}", s_slice);

               ```
               - Consider using methods on `String` like `chars` or `char_indices` rather than slicing at arbitrary indices
    
    - Why use slices as a parameters? Flexibility
        - give callers more flexibility and means functions can be used in more contexts
        - Example:
        ```
        fn main(){
            let a = [1,2,4];
            let v = vec![6,7,9];
            let v_slice=&v[..];

            only_reference_to_array(&a);
            only_reference_to_vec(&v);
            //This function can be called with some number of i32 values borrowed from either an array or a vector.
            //It can even be called with a slice of a slice.
            reference_to_array_or_vec(&a[..]);
            reference_to_array_or_vec(&v[..]);
            reference_to_array_or_vec(&v_slice[0..1]);
        }

        fn only_reference_to_array(param: &[i32; 3]){
            println!("This is an array {:?}", param);
        }

        fn only_reference_to_vec(param: &Vec<i32>){
            println!("This is an vector {:?}", param);
        }
        fn reference_to_array_or_vec(param: &[i32] ){
            println!("This is a slice {:?}", param);
        }
        ```
        - Similarly, specifying `string slices` as parameters rather than borrowing an owned `String`, functions can accept either borrowed strings and string literas. String literals can create string slices.
        ```
        fn main(){
            let s = String::from("Hello");
            let string_literal ="hello";

            either_string_or_literal(&s);
            either_string_or_literal(&string_literal);

        }
        fn either_string_or_literal(param: &str){
            println!("this is a string slice: {:?}", param);
        }
        ```
    - How does `&String` become &str? Deref trait & deref coercion
        - [Deref trait](https://doc.rust-lang.org/std/ops/trait.Deref.html)
        - Rust feature `deref coercion`: when we call a function or method, the compiler will automaticalling deference the arguments.
          If need, it would convert them to match the function parameter type.

- Borrowing and Mutability 
    - How to create and use a mutable reference: `&mut`
        ```
        $let mut x=5;
        //&mut x 
        ```
        - In functions
        - In first parameters of methods
            - Example  
            ```
            #[derive(Debug)]
            struct CarPool{
                passengers: Vec<String>,
            }

            impl CarPool{
                ///Add the named passenger to the carpool
                //we could add `&mut self` to let modify the current instance
                fn pick_up(&mut self, name: String){
                    self.passengers.push(name);
                }
            }

            fn main() {
                let mut monday_car_pool = CarPool{
                    passengers: vec![],
                };
                monday_car_pool.pick_up(String::from("Jake"));
                println!("Carpool state: {:?}", monday_car_pool);
                monday_car_pool.pick_up(String::from("Welly"));
                println!("Carpool state: {:?}", monday_car_pool);

            }
            ```
    - Borrowing Rules involving mutability 
        - You may have either
            - Many immutable references
            - One mutable reference is allowed at a time 
    - Example of a problem prevented by the borrowing rules 
        - we cannot borrow `list` as mutable because it is also borrowed as immutable.
        - The immutable borrow of list occurs in the for loop, and the mutable borrow of list occurs when we call `list.push`
            ```
            // The real problem has to do with the management of the memory backing the vector,
            // which looks something like this.
            // Here's the immutable borrow that the for loop takes and the mutable borrow that the push method takes. When you have a mutable borrow,
            // one of the aspects of the vector's data structure that could change is the pointer to the heap data.
            // If the vector has been filled to capacity,
            // and we call push to add a new value,
            // the implementation of push may need to allocate more memory, and change the pointer to point to the new storage location.
            // At this point,
            // the immutable reference that the for loop has to the vector now points to invalid memory.
            // This is memory unsafety that could cause seg faults,
            // but the Rust compiler prevents that. This particular bug is called iterator invalidation and is a problem in many languages,
            // but not in Rust. 
            fn main(){
                let mut list = vec![1,2,3];
                for i in &list{
                    println!("i is {}",i);
                    list.push(i+1);
                }
            }
            ```