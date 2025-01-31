#[cfg(test)]
mod tests {
    use log::{debug, error, info, trace, warn};

    #[test]
    fn test_log() {
        env_logger::init();

        error!("this is an error");
        warn!("this is a warning");
        info!("this is an info");
        debug!("this is a debug");
        trace!("this is a trace");
    }

    #[test]
    fn test_log4rs() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

        error!("this is an error");
        warn!("this is a warning");
        info!("this is an info");
        debug!("this is a debug");
        trace!("this is a trace");
    }
}

#[cfg(test)]
mod tests2 {
    use log::{debug, error, info, trace, warn};

    #[test]
    fn test_log() {
        env_logger::init();

        error!("this is an error");
        warn!("this is a warning");
        info!("this is an info");
        debug!("this is a debug");
        trace!("this is a trace");
    }

    #[test]
    fn test_log4rs() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

        error!("this is an error");
        warn!("this is a warning");
        info!("this is an info");
        debug!("this is a debug");
        trace!("this is a trace");
    }
}
