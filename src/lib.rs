/*!
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
*/

use log::error;
use std::fmt::Debug;

pub trait LogErrResult<T, E: Debug> {
    fn log_unwrap (self) -> T;
    fn log_expect (self, msg: &str) -> T;
    
//   fn is_ok(&self) -> bool;
//   fn is_err(&self) -> bool;
}

impl<T, E: Debug> LogErrResult<T, E> for Result<T, E> {

    /**
    `unwrap`s the `Result`, and outputs error message (in exact same style as `unwrap`) through `error!` as well.
    */

    fn log_unwrap (self) -> T {
        self.map_err(|e| {error!("called `Result::unwrap()` on an `Err` value: {:?}", e); e}).unwrap()
    }

    /**
    `expect`s the `Result`, and outputs error message (in exact same style as `expect`) through `error!` as well.
    */

    fn log_expect (self, msg: &str) -> T {
        self.map_err(|e| {error!("{}: {:?}", msg, e); e}).expect(msg)
    }
/*
    fn is_ok(&self) -> bool {
        matches!(*self, Ok(_))
    }

    fn is_err(&self) -> bool {
        !self.is_ok()
    }
*/
}

#[cfg(test)]
mod test {
    use super::*;
    
    use simplelog::SimpleLogger;

    static mut LOGGER_SET_UP: bool = false;

    fn setup_logger () {
        SimpleLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default()).unwrap();
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: \"a wild error has appeared!\"")]
    fn test_log_unwrap () {
        unsafe {if !LOGGER_SET_UP {setup_logger(); LOGGER_SET_UP = true;}}

        Result::<(), &str>::Err("a wild error has appeared!").log_unwrap();
    }

    #[test]
    #[should_panic(expected = "A wild error SHOULD appear: \"a wild error has appeared!\"")]
    fn test_log_expect () {
        unsafe {if !LOGGER_SET_UP {setup_logger(); LOGGER_SET_UP = true;}}

        (Result::<(), String>::Err("a wild error has appeared!".to_string())).log_expect("A wild error SHOULD appear");
    }
}