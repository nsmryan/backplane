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

pub enum StreamReadResult {
    BytesRead(usize),
    Finished,
    Error(String),
}

pub trait StreamRead {
    fn read_bytes(&mut self, bytes: &mut BytesMut, num_bytes: usize) -> StreamReadResult;
}

impl StreamRead for TcpStream {
    fn read_bytes(&mut self, bytes: &mut BytesMut, num_bytes: usize) -> StreamReadResult {
        read_bytes_from_reader(self, bytes, num_bytes)
    }
}

impl StreamRead for BufReader<File> {
    fn read_bytes(&mut self, bytes: &mut BytesMut, num_bytes: usize) -> StreamReadResult {
        let result = read_bytes_from_reader(self, bytes, num_bytes);

        if let StreamReadResult::BytesRead(0) = result {
            // NOTE assumes that the end of the file is the end of the stream, and no new data is
            // possible!
            return StreamReadResult::Finished;
        } else {
            return result;
        }
    }
}

impl StreamRead for UdpSocket {
    fn read_bytes(&mut self, bytes: &mut BytesMut, _num_bytes: usize) -> StreamReadResult {
        // for UDP we just read a message
        bytes.clear();
        match self.recv(bytes).map_err(|err| format!("Udp Socket Read Error: {}", err)) {
            Ok(bytes_read) => {
                return StreamReadResult::BytesRead(bytes_read);
            },

            Err(string) => {
                return StreamReadResult::Error(string);
            }
        }
    }
}


fn read_bytes_from_reader<R: Read>(reader: &mut R, bytes: &mut BytesMut, num_bytes: usize) -> StreamReadResult {
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

    match result {
        Ok(bytes_read) => {
            // if byte were read, set the BytesMut length to reflect the new data available
            bytes.truncate(old_len + bytes_read);
            return StreamReadResult::BytesRead(bytes_read);
        },

        Err(string) => {
            return StreamReadResult::Error(string);
        }
    }
}

