
pub mod common;
pub mod network;
pub mod node;
pub mod service;
pub mod tool;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::{tool};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);

        let logfile = "config/log4rs.yaml";
        if let Some(_r) = tool::l4rs::init_file(logfile) {
        } else {
            panic!("Init log file {:?} error.", logfile);
        }

        log::info!("hello");
    }
}
