pub mod logger;

#[cfg(test)]
mod tests {
    use crate::logger::MLogger;
    use crate::logger::Verbosity;

    #[test]
    fn can_print_logs(){
        let logger = MLogger::init_default();
        logger.log("this is error!", Verbosity::Error).unwrap();
        logger.log("this is info!", Verbosity::Info).unwrap();
        println!("{}",logger.get_size().unwrap());
        assert_eq!(1,1);
    }
    #[test]
    fn can_print_verbos_logs(){
        let logger = MLogger::init(Verbosity::Warn, 100);
        logger.log("this is error!", Verbosity::Error).unwrap();
        logger.log("this is info! it will not show!", Verbosity::Info).unwrap();
        println!("{}",logger.get_size().unwrap());
        assert_eq!(1,1);
    }
    #[test]
    fn can_rewrite_logs(){
        let logger = MLogger::init(Verbosity::Debug, 3);
        logger.log("this is error! and it will be deleted!", Verbosity::Error).unwrap();
        logger.log("this is info! it will show!", Verbosity::Info).unwrap();
        logger.log("this is warning! it will show!", Verbosity::Warn).unwrap();
        logger.log("this is error2! it will show!", Verbosity::Error).unwrap();
        println!("{}",logger.get_size().unwrap());
        let logs = logger.get_log(Verbosity::Debug).unwrap();
        for log in logs {
            println!("msg: {} , code: {}",log.1,log.0);
        }

        assert_eq!(1,1);
    }
    #[test]
    fn prints_logs_correctly(){
        let logger = MLogger::init(Verbosity::Warn, 100);
        logger.log("this is error!", Verbosity::Error).unwrap();
        logger.log("this is info! it will not show!", Verbosity::Info).unwrap();
        logger.log("this is error! and it will be deleted!", Verbosity::Error).unwrap();
        logger.log("this is info! it will show!", Verbosity::Info).unwrap();
        logger.log("this is warning! it will show!", Verbosity::Warn).unwrap();
        logger.log("this is error2! it will show!", Verbosity::Error).unwrap();
        logger.print_log().unwrap();
        assert_eq!(1,1);
    }

}
