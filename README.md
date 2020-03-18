# RustGettingStarted

## Hello World:

```rust

fn main() {
    println!("Hello, world!");
}

```

```bat 

(base) vivek@VIVEK-PC:/mnt/d/Git/Rust/Example1/Sample1$ cargo build --release
   Compiling Sample1 v0.1.0 (/mnt/d/Git/Rust/Example1/Sample1)
warning: crate `Sample1` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `sample1`

    Finished release [optimized] target(s) in 1.03s
(base) vivek@VIVEK-PC:/mnt/d/Git/Rust/Example1/Sample1$ ./Sample1
-bash: ./Sample1: No such file or directory
(base) vivek@VIVEK-PC:/mnt/d/Git/Rust/Example1/Sample1$ ls
Cargo.lock  Cargo.toml  src  target
(base) vivek@VIVEK-PC:/mnt/d/Git/Rust/Example1/Sample1$ cd target/
(base) vivek@VIVEK-PC:/mnt/d/Git/Rust/Example1/Sample1/target$ ls
debug  release
(base) vivek@VIVEK-PC:/mnt/d/Git/Rust/Example1/Sample1/target$ cd release/
(base) vivek@VIVEK-PC:/mnt/d/Git/Rust/Example1/Sample1/target/release$ ls
Sample1  Sample1.d  build  deps  examples  incremental
(base) vivek@VIVEK-PC:/mnt/d/Git/Rust/Example1/Sample1/target/release$ ./Sample1
Hello, world!

```


>  sudo apt install libssl-dev



> rustup target add armv7-unknown-linux-gnueabihf


> (base) vivek@VIVEK-PC$ cat >>~/.cargo/config <<EOF

> [target.armv7-unknown-linux-gnueabihf]

> linker = "arm-linux-gnueabihf-gcc"

> EOF

cargo build --target=armv7-unknown-linux-gnueabihf

## With Docker

>

> docker run --rm -v D:\Git\Rust\websocket_new\:/usr/src/myapp -w /usr/src/myapp rust:1.23.0 cargo build --release

> For bash

> docker run -it --rm -v D:\Git\Rust\websocket_new\:/usr/src/myapp -w /usr/src/myapp rust

## Using cross and docker

> cargo install cross
 
> cross test --target armv7-unknown-linux-gnueabihf

> cross build --target=mips64-unknown-linux-gnuabi64