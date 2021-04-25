[![Crates.io](https://img.shields.io/crates/v/log_err?style=flat-square)](https://crates.io/crates/log_err)
![License](https://img.shields.io/crates/l/log_err/1.0.0?style=flat-square])
[![Docs](https://img.shields.io/docsrs/log_err?style=flat-square)](https://docs.rs/log_err/)

# Log_Err
A small extension to the [log](https://crates.io/crates/log) crate, which provides two methods for `core::result::Result<T, E>` and `core::option::Option<T>`

`log_except` and `log_unwrap`, which invoke the `log::error!` macro (in case of `Result::Err`/`Option::None`) in _addition_ to unwrapping/expecting the `Result`/`Option`.

Shorthand for:

```rust
something().map_err(|e| error!("{}: {:?}", msg, e)).expect(msg)
```

Example:

```rust
let mut file = File::create("foo.txt").log_expect("Error creating file");
```
```text
# Error will be logged with the error! macro
[ERROR] Error creating file: Os { code: 2, kind: NotFound, message: "No such file or directory" }

# Main program panic'ing with same message
thread 'main' panicked at 'Error creating file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', test.rs:4:48
```
