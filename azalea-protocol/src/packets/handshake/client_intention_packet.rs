use crate::packets::ConnectionProtocol;
use packet_macros::{HandshakePacket, McBuf};
use std::hash::Hash;

#[derive(Hash, Clone, Debug, McBuf, HandshakePacket)]
pub struct ClientIntentionPacket {
    #[var]
    pub protocol_version: u32,
    pub hostname: String,
    pub port: u16,
    pub intention: ConnectionProtocol,
}
