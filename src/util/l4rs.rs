use log4rs;

/// [dependencies]
/// log4rs = "1.2.0"
/// log = "0.4.17
///
/// use like this:
///     l4rs::init_file("")

pub fn init_file(file: &str) -> Option<()>{
    let r = log4rs::init_file(file, Default::default());
    if r.is_err() {
        println!("[ERROR] {:?}", r.unwrap_err().to_string());
        return None
    }
    return Some(r.unwrap());
}
