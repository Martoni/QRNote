# QRNote
A graphical standalone program to note some text and transform it to Qrcode for «smart»phone copy/paste

![Animated GIF demonstration of QRNote](QRNote_demo.gif "Animated GIF demonstration of QRNote")

# Compile and Run

To compile and run **QRNote** you will need Cargo. It can be compiled and run
with following command :

```shell
$ cd qr-note
$ cargo run
``` 

# Install

Installation also need cargo with install command and binary path :

```shell
$ cd qr-note
$ cargo install --path .
```

## Usage (from slint template)

1. Install Rust by following the [Rust Getting Started
   Guide](https://www.rust-lang.org/learn/get-started).  Once this is done, you
   should have the `rustc` compiler and the `cargo` build system installed in
   your path.

2. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)

``` cargo install cargo-generate ```

3. Set up a sample project with this template

``` cargo generate --git
 https://github.com/slint-ui/slint-rust-template --name my-project cd
 my-project```

3. Build with cargo

``` cargo build ```

4. Run the application binary

``` cargo run ```

## Web assembly compilation

1. Install wasm :

```cargo install wasm-pack```

2. Compile QRNote

```wasm-pack build --release --target web```

3. Launch (doesn't works for the moment)

```python3 http.server```
