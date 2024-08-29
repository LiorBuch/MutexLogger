Mutex Logger
======

A logger crate that wraped with mutex guard to allow logging from multithreading.

## Example

```rust

    let logger = Logger::init(Verbosity::Warn, 100);
    logger.log("this is error log!", Verbosity::Error).unwrap();
    logger.log("this is info log! it will not show!", Verbosity::Info).unwrap();
    println!("{}",logger.get_size().unwrap());

```