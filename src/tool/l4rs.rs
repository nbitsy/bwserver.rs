
//! It's a simple wrapper for log4rs
//!
//! [dependencies]
//! log4rs = "1.2.0"
//! log = "0.4.17
//!
//! use bwserver;
//! use log;
//!
//! let logfile = "config/log4rs.yaml";
//!
//! if bwserver::tool::l4rs::init_file(logfile) == None {
//!     panic!("Init log file {:?} error.", logfile);
//! )
//!
//! log::info!("hello");

use log4rs;

/// We should provide the config `file` in yaml format
pub fn init_file(file: &str) -> Option<()>{
    let r = log4rs::init_file(file, Default::default());
    if r.is_err() {
        println!("[ERROR] {:?}", r.unwrap_err().to_string());
        return None
    }

    return Some(r.unwrap());
}
