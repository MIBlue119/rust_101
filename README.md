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

## Resources
- [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) 
- [Rust By Example](https://doc.rust-lang.org/book/title-page.html)
