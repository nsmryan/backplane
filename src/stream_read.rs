use std::fs::File;
use std::io::{Read, BufReader};
use std::net::{TcpStream, UdpSocket};
use std::borrow::BorrowMut;

use bytes::BytesMut;


pub trait StreamRead {
    fn read_bytes(&mut self, bytes: &mut BytesMut, num_bytes: usize) -> Result<usize, String>;
}

impl StreamRead for TcpStream {
    fn read_bytes(&mut self, bytes: &mut BytesMut, num_bytes: usize) -> Result<usize, String> {
        read_bytes_from_reader(self, bytes, num_bytes)
    }
}

impl StreamRead for BufReader<File> {
    fn read_bytes(&mut self, bytes: &mut BytesMut, num_bytes: usize) -> Result<usize, String> {
        read_bytes_from_reader(self, bytes, num_bytes)
    }
}

impl StreamRead for UdpSocket {
    fn read_bytes(&mut self, bytes: &mut BytesMut, _num_bytes: usize) -> Result<usize, String> {
        // for UDP we just read a message
        bytes.clear();
        self.recv(bytes).map_err(|err| format!("Udp Socket Read Error: {}", err))
    }
}

fn read_bytes_from_reader<R: Read>(reader: &mut R, bytes: &mut BytesMut, num_bytes: usize) -> Result<usize, String> {
    let current_len = bytes.len();

    bytes.reserve(num_bytes);

    let mut_bytes: &mut [u8] = bytes.borrow_mut();
    reader.read_exact(&mut mut_bytes[current_len..(current_len + num_bytes)])
          .map_err(|err| format!("Stream Read Error: {}", err))?;

    Ok(num_bytes)
}

