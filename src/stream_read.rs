use std::fs::File;
use std::io::{Read, BufReader};
use std::net::{TcpStream, UdpSocket};
use std::borrow::BorrowMut;

use bytes::BytesMut;


// TODO this API does not make blocking vs non-block calls apparent
// ideally there would be a timeout provided, which could be 0 (non-blocking)
// a timeout, or infinite (block until data is available). This would cover the
// case of files which are being written as well as read.
// TODO this error return of String should be replaced with a error handling strategy
// TODO this should include serial reading
// TODO this might include stdin reading
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
    let old_len = bytes.len();
    let new_len = old_len + num_bytes;

    // ensure that there is room for the request number of bytes
    bytes.reserve(num_bytes);

    // NOTE this zeroing of memory could be avoided with a use of the unsafe function
    // set_len. This has not been done to avoid use of 'unsafe'.
    bytes.resize(new_len, 0);

    // retrieve the underlying byte buffer
    let mut_bytes: &mut [u8] = bytes.borrow_mut();

    // read up to num_bytes bytes from the reader
    let result = reader.read(&mut mut_bytes[old_len..(old_len + num_bytes)])
                       .map_err(|err| format!("Stream Read Error: {}", err));

    // if byte were read, set the BytesMut length to reflect the new data available
    if let Ok(bytes_read) = result {
        bytes.truncate(old_len + bytes_read);
    }

    return result;
}

