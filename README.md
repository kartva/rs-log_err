[![Crates.io](https://img.shields.io/crates/v/log_err?style=flat-square)](https://crates.io/crates/log_err)

# Log_Err
A small extension to the [log](https://crates.io/crates/log) crate, which provides two methods for `core::result::Result<T, E>` :

`log_except` and `log_unwrap`, which invoke the `log::error!` macro (in case of `Result::Err`) in _addition_ to unwrapping/expecting the `Result`.

Shorthand for:

```rust
something().map_err(|e| error!("{}: {:?}", msg, e)).expect(msg)
```

Example:

```rust
let mut file = File::create("foo.txt").log_expect("Error creating file");
```
```
# Will output on your logger and on the main program as well:

# Or however your logger formats messages

[ERROR] Error creating file: Os { code: 2, kind: NotFound, message: "No such file or directory" }

# Main program panic'ing with same message

thread 'main' panicked at 'Error creating file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', test.rs:4:48
```
