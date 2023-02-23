
pub struct ProtocolMessage {

}

#[repr(packed)]
pub struct ProtocolNode {
    /// raw protocol
    message: ProtocolMessage,
    /// source node id
    src: u32,
    /// destination node id
    dst: u32,
}

impl ProtocolNode {
    pub const fn new() -> Self {
        return ProtocolNode {
            message: ProtocolMessage::new(),
            src: 0,
            dst: 0,
        };
    }

    pub fn to_string(&self) -> String {
        format!("[{:?}] src:[{:?}] dst:[{:?}]", self.message.to_string(), self.src as u32, self.dst as u32)
    }
}