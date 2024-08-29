use std::{
    collections::VecDeque,
    fmt::Display,
    sync::{Arc, Mutex},
};

/// `Verbosity` is the enum that declares the scope of each log.   
/// Don't use [`Verbosity::Silent`] as a log condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    Silent,
    Error,
    Warn,
    Info,
    Debug,
}
impl Display for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verbosity::Silent => write!(f, "Silent"),
            Verbosity::Error => write!(f, "Error"),
            Verbosity::Warn => write!(f, "Warn"),
            Verbosity::Info => write!(f, "Info"),
            Verbosity::Debug => write!(f, "Debug"),
        }
    }
}
/// `MLogger` is a struct to control the logging logic and hold the logs.
///
/// # Parameters
/// - `verbosity`: Controls the logger print logic; it will print only if the verbosity of the log is within the threshold.   
///   The threshold is defined by the Enum [`Verbosity`].   
/// - `max_size: usize`: Controls the maximum number of logs that can exist in the instance; will push out old logs above that limit.   
/// - `pool: VecDeque<(u32, String, Verbosity)>`: Mutex-controlled double-ended queue that holds all the logs.   
/// - `counter: u32`: Counts the log ID; each log in a session gets a counted ID, so index 1 does not imply id == 1.
///
/// # Log Entry
/// A log entry is a tuple of `(u32, String, Verbosity)`, where:   
/// - `u32`: The counted ID, assigned automatically.   
/// - `String`: The log message itself.   
/// - `Verbosity`: Sets the [`Verbosity`] level of the log.
///
/// # Initialization
/// To get a `MLogger` instance, call [`MLogger::init_default()`] or [`MLogger::init()`] to control the verbosity level and maximum pool size.
pub struct MLogger {
    verbosity: Verbosity,
    max_size: usize,
    pool: Arc<Mutex<VecDeque<(u32, String, Verbosity)>>>,
    counter: Arc<Mutex<u32>>,
}
impl MLogger {
    /// This method will create a [`MLogger`] instance by its default values `1000` for the pool and [`Verbosity::Debug`] for verbosity.
    pub fn init_default() -> MLogger {
        return MLogger {
            verbosity: Verbosity::Debug,
            max_size: 1000,
            pool: Arc::new(Mutex::new(VecDeque::new())),
            counter: Arc::new(Mutex::new(0)),
        };
    }
    /// Creates a [`MLogger`] instance, allowing control over the pool size and verbosity level.
    ///
    /// # Param
    /// - `verbosity: Verbosity`: Sets the `MLogger` verbosity level.   
    /// - `max_size: usize`: Sets the maximum number of logs until the logger will push out old logs.
    pub fn init(verbosity: Verbosity, max_size: usize) -> MLogger {
        return MLogger {
            verbosity: verbosity,
            max_size: max_size,
            pool: Arc::new(Mutex::new(VecDeque::new())),
            counter: Arc::new(Mutex::new(0)),
        };
    }
    /// Inserts a log into the MLogger; it will print if the verbosity predicate matches.
    ///
    /// # Param
    /// - `log: &str`: The message to be logged.   
    /// - `verbosity: Verbosity`: The message verbosity level; it will affect the appearance of the message.
    ///
    /// Returns a `Result` with an error message as a `String` or `()` on success.
    pub fn log(&self, log: &str, verbosity: Verbosity) -> Result<(), String> {
        let mut pool = self
            .pool
            .lock()
            .map_err(|_| "pool lock failed!".to_string())?;
        let mut counter = self
            .counter
            .lock()
            .map_err(|_| "counter lock failed!".to_string())?;
        let log_entry = (*counter, log.to_string(), verbosity);
        if log_entry.2 <= self.verbosity {
            println!("{}", log_entry.1.clone());
        }
        pool.push_front(log_entry);
        if pool.len() > self.max_size {
            pool.pop_back();
        }
        *counter += 1;
        Ok(())
    }
    /// Retrieves an entry from the logger.   
    /// *Note that index indicates the recency of a log; if a log gets pushed out, it is gone...*
    ///
    /// # Param
    /// - `index: usize`: The log index in the pool.
    ///
    /// Returns a `Result` with an error message as a `String` or the log entry tuple on success.
    pub fn get_entry(&self, index: usize) -> Result<(u32, String, Verbosity), String> {
        let pool = self
            .pool
            .lock()
            .map_err(|_| "pool lock failed".to_string())?;
        pool.get(index)
            .cloned()
            .ok_or_else(|| "index out of bounds".to_string())
    }
    /// Retrieves the current size of the pool.
    ///
    /// Returns a `Result` with an error message as a `String` or the size (`usize`) on success.
    pub fn get_size(&self) -> Result<usize, String> {
        let pool = self
            .pool
            .lock()
            .map_err(|_| "pool lock failed".to_string())?;
        Ok(pool.len())
    }
    /// Retrieves all entries from the logger with a filter.   
    /// *Use `filter=[Verbosity::Debug]` to get all the logs.*
    ///
    /// # Param
    /// - `filter: Verbosity`: The predicate to limit the scope of the logs.
    ///
    /// Returns a `Result` with an error message as a `String` or all the log entries that match the predicate on success.
    pub fn get_log(&self, filter: Verbosity) -> Result<Vec<(u32, String, Verbosity)>, String> {
        let pool = self
            .pool
            .lock()
            .map_err(|_| "pool lock failed!".to_string())?;
        let filtered_logs: Vec<(u32, String, Verbosity)> =
            pool.iter().filter(|log| log.2 <= filter).cloned().collect();
        Ok(filtered_logs)
    }
    /// Retrieves a slice from the logs.
    /// *Can fail if indices are incorrect.*
    ///
    /// # Param
    /// - `start_index: usize`: The start index of the slice.   
    /// - `end_index: usize`: The end index of the slice.   
    /// - `filter: Verbosity`: The predicate to limit the scope of the logs.
    ///
    /// Returns a `Result` with an error message as a `String` or the log entries slice on success.
    pub fn get_entries(
        &self,
        start_index: usize,
        end_index: usize,
        filter: Verbosity,
    ) -> Result<Vec<(u32, String, Verbosity)>, String> {
        let pool = self
            .pool
            .lock()
            .map_err(|_| "pool lock failed!".to_string())?;
        let sub_pool = pool
            .range(start_index..end_index)
            .filter(|log| log.2 <= filter)
            .cloned()
            .collect();
        Ok(sub_pool)
    }
    /// Prints all the logs.
    ///
    /// Returns a `Result` with an error message as a `String` or `()` on success.
    pub fn print_log(&self) -> Result<(), String> {
        let pool = self
            .pool
            .lock()
            .map_err(|_| "pool lock failed!".to_string())?;
        for entry in pool.iter() {
            println!("id:{} {} {}", entry.0, entry.2, entry.1);
        }
        Ok(())
    }
    /// Prints specific log levels.
    ///
    /// # Param
    /// - `predicate: Verbosity`: The predicate to limit the scope of the logs.
    ///
    /// Returns a `Result` with an error message as a `String` or `()` on success.
    pub fn print_log_level(&self, predicator: Verbosity) -> Result<(), String> {
        let pool = self
            .pool
            .lock()
            .map_err(|_| "pool lock failed!".to_string())?;
        let filtered_logs: Vec<(u32, String, Verbosity)> = pool
            .iter()
            .filter(|log| log.2 == predicator)
            .cloned()
            .collect();
        for entry in filtered_logs {
            println!("{} {} {}", entry.0, entry.2, entry.1);
        }
        Ok(())
    }
}
