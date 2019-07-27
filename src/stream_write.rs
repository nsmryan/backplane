use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpStream, UdpSocket, SocketAddrV4};


// TODO this should include serial writing
// TODO this might include stdin writing
pub trait StreamWrite {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, String>;
}

impl StreamWrite for File {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, String> {
        self.write_all(&bytes)
            .map_err(|err| format!("IO error {}", err))
            .map(|_| bytes.len())
    }
}

// TODO make this a Udp stream type instead of a tuple
impl StreamWrite for (UdpSocket, SocketAddrV4) {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, String> {
        self.0.send_to(&bytes, &self.1)
                .map_err(|err| format!("IO error {}", err))
    }
}

impl StreamWrite for TcpStream {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, String> {
        self.write_all(&bytes)
            .map_err(|err| format!("IO error {}", err))
            .map(|_| bytes.len())
    }
}

