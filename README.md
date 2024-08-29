Mutex Logger
======

A logger crate that uses mutex guards to allow logging in multithreading to prevent data racing.

## Example

```rust

    let logger = Logger::init(Verbosity::Warn, 100);
    logger.log("this is error log!", Verbosity::Error).unwrap();
    logger.log("this is info log! it will not show!", Verbosity::Info).unwrap();
    println!("{}",logger.get_size().unwrap());

```