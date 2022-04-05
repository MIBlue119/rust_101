# Rust 101 

In order tor know some repos implemented in Rust, I start to learn Rust in the Ching Ming Festival.

## Intro 

- Why do we need Rust? 
    - Rust complier plays a gatekeeper role to find elusive bugs
    - It could be use at cli/ web/ DevOps tool/ embedded devices/ml/ audio analysis
    - An ahead-of time compiled language 
- How could we install the environment to run Rust?
    - Install `rustup` at Linux/MacOS
        ```
        $curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
        ```
    - Update 
        ```
        $rustup update
        ```
    - Uninstall 
        ```
        $rustup self uninstall
        ```
    - Check the version 
        ```
        $rustc --version
        ```
- Compile the code, but if the project is big, we may use `Cargo` 
  ```
  $rustc xxx.rs
  ```
  - Run the code
    ```
    ./xxx
    ```
- How could we run the package manager `Cargo` for rust?
    - `Cargo` is Rust build system and package manager.
        - It could build your code.
        - It could download the dependency libraries and build that.
        - It comes installed with Rust
        - Check your cargo version
            ```
            $cargo --version
            ```
    - Use cargo to create a project
        ```
        $ cargo new projects/hello_cargo
        ```
    - Build the project
        ```
        $cargo build 
        ```
        - this build would generate a compiled file at `./target/debug/`
        - If you want to build a releas version and generate it at `./target/release/`
        ```
        $cargo buld --release
        ```
    -Build and Run the project
        ```
        $cargo run 
        ```
    - Check the project could be compiled
        ```
        $cargo check 
        ```
    - Other: build a rust project from github
    ```
    $git clone example.org/someproject
    $cd someproject
    $cargo build 
    ```
- How to activate test block code?
    ```
    $cargo test
    ```
- Use an external crate to get mode functionality
    - `Crate`: a collection of Rust source code files. 
        - We could ues some crates at [Crates.io](https://crates.io/)
        - We need to add the `crate` at your `Cargo.toml` `[dependencies]` 
            - Tell the Cargo which external crates your project depend on 
            - You have to fill the crate's version 
            ```
            [dependencies]
            rand = "0.8.4" # Add the dependency library to build random numbers
            #The number 0.8.3 is actually shorthand for ^0.8.3, which means any version that is at least 0.8.3 but below 0.9.0. Cargo considers these versions to have public APIs compatible with version 0.8.3, 
            #and this specification ensures you’ll get the latest patch release that will still compile with the code in this chapter. 
            ```
        - `Crago` checks `[dependencies]` and fetches the latest versions of dependency needs from the registry
    - Update a `Crate` to get a new version
        ```
        $cargo update 
        ```
- Create the documentation provided by all of your dependencies 
    ```
    $cargo doc --open
    ```
## Others
- `mod.rs` is like `__init__.py` in the python project
    - The complier would seem the folder with the file `mode.rs` as a module.
## Resources

### Tutorials
- [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) 
- [Rust By Example](https://doc.rust-lang.org/book/title-page.html)
- [The Cago Book](https://doc.rust-lang.org/cargo/index.html): A book about Cargo from official
- [Rustlings](https://github.com/rust-lang/rustlings): Interactive exerices according to Rust Documents's module

### Code an Tools
- [Crates.io](https://crates.io/): is where people in the Rust ecosystem post their open source Rust projects for others to use