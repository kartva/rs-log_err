use log::error;
use std::fmt::Debug;

pub trait LogErrResult<T, E: Debug> {
    fn log_unwrap (self) -> T;
    fn log_expect (self, msg: &str) -> T;
    
//   fn is_ok(&self) -> bool;
//   fn is_err(&self) -> bool;
}

impl<T, E: Debug> LogErrResult<T, E> for Result<T, E> {
    fn log_unwrap (self) -> T {
        self.map_err(|e| {error!("called `Result::unwrap()` on an `Err` value: {:?}", e); e}).unwrap()
    }

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