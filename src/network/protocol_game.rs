
pub struct ProtocolGame {
    /// command id
    pub cmd: u32,
    /// user id
    pub uid: u64,
    /// client serialization number
    pub csn: u64,
    /// server serialization number
    pub ssn: u64,
}

impl ProtocolGame {
    pub const fn new() -> Self {
        ProtocolGame {
            cmd: 0,
            uid: 0,
            csn: 0,
            ssn: 0,
        }
    }
}