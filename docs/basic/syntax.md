


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

