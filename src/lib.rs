/*!
# Log_Err
A small extension to the [log](https://crates.io/crates/log) crate, which provides two methods for `core::result::Result<T, E>` and `core::option::Option<T>`

`log_except` and `log_unwrap`, which invoke the `log::error!` macro (in case of `Result::Err`) in _addition_ to unwrapping/expecting the `Result`.

Shorthand for:

```should_panic
# use log::error;
# fn something() -> Result<(), &'static str> {Err("there was some problem")}
# let msg = "Some message";
something().map_err(|e| error!("{}: {:?}", msg, e)).expect(msg)
```

Example:

```should_panic
# use std::fs::File;
# use log::error;
# use log_err::LogErrResult;
let mut file = File::open("foo.txt").log_expect("Error creating file");
```
```text
# Error will be logged with the error! macro
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
}

pub trait LogErrOption<T> {
    fn log_unwrap (self) -> T;
    fn log_expect (self, msg: &str) -> T;
}

impl<T> LogErrOption<T> for Option<T> {

    /**
    `unwrap`s the `Option`, and outputs error message (in exact same style as `unwrap`) through `error!` as well.
    */

    fn log_unwrap (self) -> T {
        match self {
            Some (n) => n,
            None => {
                error!("called `Option::unwrap()` on a `None` value");
                self.unwrap()
            }
        }
    }

    /**
    `expect`s the `Option`, and outputs error message (in exact same style as `expect`) through `error!` as well.
    */

    fn log_expect (self, msg: &str) -> T {
        match self {
            Some (n) => n,
            None => {
                error!("{}", msg);
                self.expect(msg)
            },
        }
    }
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
    #[should_panic(expected = "A wild error SHOULD appear")]
    fn test_option_log_expect () {
        unsafe {if !LOGGER_SET_UP {setup_logger(); LOGGER_SET_UP = true;}}

        Option::<()>::None.log_expect("A wild error SHOULD appear");
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_option_log_unwrap () {
        unsafe {if !LOGGER_SET_UP {setup_logger(); LOGGER_SET_UP = true;}}

        Option::<()>::None.log_unwrap();
    }
}