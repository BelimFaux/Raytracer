# GFX - lab 3

## Claim

- T1, T2, T3, T4, T5

## Tested Environments

Tested on

- EndeavourOS Linux x86_64 (Kernel: 6.14.9-arch1-1)
- Arch Linux x86_64 (Kernel: 6.14.9-arch1-1)

With

- Cargo/rustc 1.87.0

## Additional and general remarks

- The input file can be given via a commandline argument. So the program can be compiled and run with the following command:

```sh
cargo run --release -- scenes/example1.xml

# or

cargo build --release
./target/release/lab3 scenes/example1.xml
```

- the binary can be optionally compiled without png support. In that case, the images are exported as ppm files.

```sh
cargo build --no-default-features --release
```

- The program will save the resulting image files with the name specified in the input file in an `output` directory
  - If this directory does not exist, the program will fail
