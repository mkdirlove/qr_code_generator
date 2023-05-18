<h1 align="center">
  <br>
  <a href="https://github.com/mkdirlove/qr_code_generator"><img src="https://github.com/mkdirlove/qr_code_generator/blob/main/logo.png" alt="qr_code_generator"></a>
  <br>
  A simple QR Code generator written in Rust.
  <br>
</h1>

#### Installation

Copy-paste this into your terminal:

```
git clone https://github.com/mkdirlove/qr_code_generator.git
```
```
cargo update
```
```
cargo run -- -h
```

#### Usage
```
$ cargo run -- -h
 
Please provide a value for the -t or --text flag.
```
#### Sample Usage #1

Generate QR Code with text

```
$ cargo run -- -t "Something..." 
```
```
$ cargo run -- --text "Something..." 
```

#### Sample Usage #2

Generate QR Code with link

```
$ cargo run -- -t "https://github.com/mkdirlove"
```
```
$ cargo run -- --text "https://github.com/mkdirlove"
```
