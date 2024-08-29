use std::{collections::VecDeque, sync::{Arc, Mutex}};


#[derive(Debug,Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    Silent,
    Error,
    Warn,
    Info,
    Debug,
}

pub struct Logger {
    verbosity: Verbosity,
    max_size: usize,
    pool: Arc<Mutex<VecDeque<(u32,String,Verbosity)>>>,
    counter:Arc<Mutex<u32>>,
}
impl Logger {
    pub fn init_default() -> Logger {
        return Logger { verbosity: Verbosity::Debug, max_size: 1000, pool: Arc::new(Mutex::new(VecDeque::new())),counter: Arc::new(Mutex::new(0)) }
    }
    pub fn init(verbosity: Verbosity, max_size:usize) -> Logger {
        return Logger { verbosity:verbosity, max_size:max_size, pool: Arc::new(Mutex::new(VecDeque::new())),counter: Arc::new(Mutex::new(0))}
    }
    pub fn log(&self, log: &str,verbosity: Verbosity) -> Result<(),String> {
        let mut pool = self.pool.lock().map_err(|_| "pool lock failed!".to_string())?;
        let mut counter = self.counter.lock().map_err(|_| "counter lock failed!".to_string())?;
        let log_entry = (counter.clone(),log.to_string(),verbosity);
        if log_entry.2 <= self.verbosity {
            println!("{}", log_entry.1.clone());
        }
        pool.insert(0, log_entry);
        if pool.len() > self.max_size {
            pool.truncate(self.max_size);
        }
        *counter+=1;
        Ok(())
    }
    pub fn get_entry(&self,index:usize) -> Result<(u32,String,Verbosity),String>{
        let pool = self.pool.lock().map_err(|_| "pool lock failed".to_string())?;
        pool.get(index).cloned().ok_or_else(|| "index out of bounds".to_string())
    }
    pub fn get_size(&self) -> Result<usize,String>{
        let pool = self.pool.lock().map_err(|_| "pool lock failed".to_string())?;
        Ok(pool.len())
    }
    pub fn get_log(&self,filter:Verbosity) -> Result<Vec<(u32,String,Verbosity)>,String>{
        let pool = self.pool.lock().map_err(|_| "pool lock failed!".to_string())?;
        let filtered_logs: Vec<(u32, String, Verbosity)> = pool
            .iter()
            .filter(|log| log.2 <= filter)
            .cloned()
            .collect();
        Ok(filtered_logs)
    }
    pub fn get_entries(&self,start_index:usize,end_index:usize)->  Result<Vec<(u32,String,Verbosity)>,String>{
        let pool = self.pool.lock().map_err(|_| "pool lock failed!".to_string())?;
        let sub_pool = pool.range(start_index..end_index).cloned().collect();
        Ok(sub_pool)
    }
}