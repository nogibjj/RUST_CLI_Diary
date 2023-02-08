#  IDS721 Project1 Commands Line Tool using RUST

## Goal of the Project

The project aims to build a useful command line tool to quickly record your diary with time stamp and you can recall it whenever you want;)

## Diagram
![ids7021_project1](https://user-images.githubusercontent.com/112578755/217579770-e7f7b6c2-6687-49e1-8f31-910823bb33ab.jpg)

## Process

* First, main.rs file is formulated with commands:

    `cargo init name -- "diary"`

* Second, you wrote the needed RUST code, see main.rs and lib.rs, and generate the `diary.txt`. 
* We also need two other dependencies added to `Cargo.toml` file:

    `clap = {version="4.0.32", features=["derive"]}`
    
    `chrono = "0.4.19"`

* To run it in terminal, type:

    To write your diary   `cargo run -- write "type your diary"`

    To read your diary  `cargo run -- read`
 
 * It's also pushed to DockerHub, so you can simply run it by typing:
 
    `docker run emmawang00/rust_cli_diary write "type your thoughts here"`

    `docker run emmawang00/rust_cli_diary read` to see what you've been writing about.
 
 * If you want to build a docker image on your own, type:

    `docker build . -t diary`

## Example Output
<img width="917" src="https://user-images.githubusercontent.com/112578755/217573198-c6db4cf5-ef6f-4f65-bd66-fdc96c442bd5.png">

<img width="917" src="https://user-images.githubusercontent.com/112578755/217573353-4b35f389-3a90-4bff-9831-a15c958c4896.png">

## Demo

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
