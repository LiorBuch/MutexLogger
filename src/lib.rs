pub mod logger;

#[cfg(test)]
mod tests {
    use crate::logger::Logger;
    use crate::logger::Verbosity;

    #[test]
    fn can_print_logs(){
        let logger = Logger::init_default();
        logger.log("this is error!", Verbosity::Error).unwrap();
        logger.log("this is info!", Verbosity::Info).unwrap();
        println!("{}",logger.get_size().unwrap());
        assert_eq!(1,1);
    }
    #[test]
    fn can_print_verbos_logs(){
        let logger = Logger::init(Verbosity::Warn, 100);
        logger.log("this is error!", Verbosity::Error).unwrap();
        logger.log("this is info! it will not show!", Verbosity::Info).unwrap();
        println!("{}",logger.get_size().unwrap());
        assert_eq!(1,1);
    }
    #[test]
    fn can_rewrite_logs(){
        let logger = Logger::init(Verbosity::Warn, 100);
        logger.log("this is error!", Verbosity::Error).unwrap();
        logger.log("this is info! it will not show!", Verbosity::Info).unwrap();
        println!("{}",logger.get_size().unwrap());
        assert_eq!(1,1);
    }

}
