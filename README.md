Mutex Logger
======

A Rust logging crate designed for multithreaded environments. `MLogger` uses mutex guards to ensure thread-safe logging, preventing data races when logging from multiple threads concurrently.

## Features
 - Thread-Safe Logging: Uses Mutex to protect internal data structures, making it safe to log from multiple threads.
 - Configurable Verbosity Levels: Control which logs are shown based on their severity level (e.g., Error, Warn, Info, Debug).
 - Log Retention: Set a maximum number of log entries to keep, automatically discarding the oldest logs when the limit is reached.
 - Flexible API: Easily log messages, retrieve logs, and print logs at different verbosity levels.

## Verbosity Levels
`MLogger` supports different verbosity levels to control which logs are displayed:

 - Silent: No logs are shown.
 - Error: Only error-level logs are shown.
 - Warn: Warning and error-level logs are shown.
 - Info: Information, warning, and error-level logs are shown.
 - Debug: All logs are shown, including debug-level logs.

## Example



```rust

    let logger = Logger::init(Verbosity::Warn, 100);
    logger.log("this is error log!", Verbosity::Error).unwrap();
    logger.log("this is info log! it will not show!", Verbosity::Info).unwrap();
    println!("{}",logger.get_size().unwrap());

```