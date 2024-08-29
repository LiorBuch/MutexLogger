use std::{
    collections::VecDeque,
    fmt::Display,
    sync::{Arc, Mutex},
};

/// `Verbosity` is the enum that declares the scope of each log.   
/// Dont use [`Verbosity::Silent`] as a log condition.
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
/// `Logger` is a struct to control the logging logic and hold the logs.
///
/// # Params
/// @verbosity -> Controls the logger print logic, it will print only if the verbosity of the log is in the threshold.   
/// The treshold is defined by the Enum [`Verbosity`].   
/// @max_size:`usize` -> Controls the maximum amount of logs that can exist in the instance, will push out old logs above that limit.   
/// @pool:`VecDeque<(u32,String,Verbosity)` -> Mutex controled 2-Way queue that holds all the logs.   
/// @counter:`u32` -> counts the log id, each log in a session gets a counted id, so index 1 does not implies id == 1.
///
/// # Log Entry
/// A log entry is a tuple of(u32,String,Verbosity),where:   
/// @u32 -> Is for the counted id, assigned automaticaly.   
/// @String -> Is for the Log message itself.   
/// @Verbosity -> Sets the [`Verbosity`] level of the log.
///
/// # Initialization
/// To get a Logger instance call [`Logger::init_default()`] or [`Logger::init()`] to control the verbosity level and max pool size.
pub struct Logger {
    verbosity: Verbosity,
    max_size: usize,
    pool: Arc<Mutex<VecDeque<(u32, String, Verbosity)>>>,
    counter: Arc<Mutex<u32>>,
}
impl Logger {
    /// This method will create a [`Logger`] instance by its default values `1000` for the pool and [`Verbosity::Debug`] for verbosity.
    pub fn init_default() -> Logger {
        return Logger {
            verbosity: Verbosity::Debug,
            max_size: 1000,
            pool: Arc::new(Mutex::new(VecDeque::new())),
            counter: Arc::new(Mutex::new(0)),
        };
    }
    /// This method will create a [`Logger`] instance but will allow you to control the pool size and verbosity level.
    ///
    /// @verbosity:`Verbosity` -> Sets the Logger verbosity level.   
    /// @max_size:`usize` -> Sets the maximum amount of logs unstill the logger will push out old logs.
    pub fn init(verbosity: Verbosity, max_size: usize) -> Logger {
        return Logger {
            verbosity: verbosity,
            max_size: max_size,
            pool: Arc::new(Mutex::new(VecDeque::new())),
            counter: Arc::new(Mutex::new(0)),
        };
    }
    /// Call this method to insert a log to the Logger, it will print if the verbosity predicator match.
    ///
    /// @log:`&str` -> The message to be logged.   
    /// @verbosity:`Verbosity` -> The message verbosity level. It will effect the appearnce of the message.
    ///
    /// Returns -> String with error or void on sucess.
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
    /// Call this method to get an entry out of the logger.   
    /// *Note that index means the recenty of a log, if a log gets pushed out, its gone...*
    ///
    /// @index:`usize` -> The log index in the pool.
    ///
    /// Returns -> `String` with error, the log entry tuple on sucess.
    pub fn get_entry(&self, index: usize) -> Result<(u32, String, Verbosity), String> {
        let pool = self
            .pool
            .lock()
            .map_err(|_| "pool lock failed".to_string())?;
        pool.get(index)
            .cloned()
            .ok_or_else(|| "index out of bounds".to_string())
    }
    /// Call this to get the current size of the pool.
    ///
    /// Returns -> `String` with error or the size:`usize` on sucess.
    pub fn get_size(&self) -> Result<usize, String> {
        let pool = self
            .pool
            .lock()
            .map_err(|_| "pool lock failed".to_string())?;
        Ok(pool.len())
    }
    /// Call this method to get all entries out of the logger with a filter.   
    /// *Use filter=[`Verbosity::Debug`] to get all the logs.*
    ///
    /// @filter:`Verbosity` -> The predicator to limit the scope of the logs.
    ///
    /// Returns -> `String` with error,all the log entries that match the predicator on sucess.
    pub fn get_log(&self, filter: Verbosity) -> Result<Vec<(u32, String, Verbosity)>, String> {
        let pool = self
            .pool
            .lock()
            .map_err(|_| "pool lock failed!".to_string())?;
        let filtered_logs: Vec<(u32, String, Verbosity)> =
            pool.iter().filter(|log| log.2 <= filter).cloned().collect();
        Ok(filtered_logs)
    }
    /// Call this method to get a slice from the logs.
    /// *Can fail if indices are incorrect.*
    ///
    /// @start_index:`usize` -> The start index of the slice.   
    /// @end_index:`usize` -> The start index of the slice.   
    /// @filter:`Verbosity` -> The predicator to limit the scope of the logs.
    ///
    /// Returns -> `String` with error, the log entries slice on sucess.
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
    /// Call this method to print all the log.
    ///
    /// Returns -> `String` with error, void on sucess.
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
    /// Call this method to print spesific log level.
    ///
    /// @predicator:`Verbosity` -> The predicator to limit the scope of the logs.
    ///
    /// Returns -> `String` with error, void on sucess.
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
