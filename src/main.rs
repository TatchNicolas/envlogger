use rand::Rng;
use std::{thread, time};

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


// TODO Try using enums for log level
#[derive(Serialize, Deserialize)]
struct LogRecord {
    level: String,
    message: String,
}

fn main() {
    let default_logs = vec![
        // "foo".to_string() v. String::from("foo") ?
        LogRecord { level: "DEBUG".to_string(), message: "Debugging is hard.".to_string() },
        LogRecord { level: "INFO".to_string(), message: "Infomation is useful.".to_string() },
        LogRecord { level: "WARN".to_string(), message: "Warning is surprising.".to_string() },
        LogRecord { level: "ERROR".to_string(), message: "Error is inevitable.".to_string() },
    ];

    // TODO load configuration
    // TODO set interval
    // TODO set log messages via config file
    // TODO set log messages via environment variables
    
    // start infinite loop
    let interval = time::Duration::from_secs(3);

    loop {
        let index = rand::thread_rng().gen_range(0, default_logs.len());
        let log_line = &default_logs[index];
        let log_line = serde_json::to_string(log_line);
        let log_line = match log_line {
            Ok(message) => message,
            Err(error) => panic!("Failed to deserialize: {:?}", error),
        };
        
        println!("{}", log_line);

        // What happens to its ownership...?
        thread::sleep(interval);
    }
}
