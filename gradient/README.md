# gradient

This is my first project in rust. Trying to get text to print out in a pretty gradient :p

## Setup

Make sure to have installed `Cargo` before beginning a project.

Can create a new project by going `cargo new [project name]`

## Guide

I am using the crate 'colorful' which is just a package to do some pretty text stuff.

I had to add it as a dependency within the `Cargo.toml` file.

```git
[dependencies]
+ colorful = "0.2.1"
```

## Running the program

Just use the following in the project folder:

```sh
cargo run
```