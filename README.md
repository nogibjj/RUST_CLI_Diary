#  IDS721 Project1 Commands Line Tool using RUST

## Goal of the Project

The project aims to build a useful command line tool to quickly record your diary with time stamp and you can recall it whenever you want;)

## Diagram

## Process

* First, main.rs file is formulated with commands:

    `cargo init name -- "diary"`

* Second, you wrote the needed RUST code, see main.rs and lib.rs. 
* We also need two other dependencies added to `Cargo.toml` file:

    `clap = {version="4.0.32", features=["derive"]}`
    `chrono = "0.4.19"`

* To run it in terminal, type:

 To write your diary   `cargo run -- write "type your diary"`
 To read your diary  `cargo run -- read`

## Demo

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
