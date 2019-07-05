extern crate serde;
#[macro_use] extern crate serde_derive;

extern crate num;
#[macro_use] extern crate num_derive;

pub mod stream_read;
pub mod stream_write;

use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream, UdpSocket, SocketAddrV4};
use std::error::Error;
use std::str::FromStr;

use bytes::BytesMut;

use crate::stream_write::*;
use crate::stream_read::*;


/// The stream settings are all the settings for all stream types
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamSettings {
    #[serde(default)]
    pub file: FileSettings,

    #[serde(default)]
    pub tcp_client: TcpClientSettings,

    #[serde(default)]
    pub tcp_server: TcpServerSettings,

    #[serde(default)]
    pub udp: UdpSettings,
}

impl StreamSettings {
    pub fn open_input(&self, input_option: &StreamOption) -> Result<ReadStream, String> {
        let result;

        match input_option {
            StreamOption::File => {
                result = self.file.open_read_stream();
            },

            StreamOption::TcpClient => {
                result = self.tcp_client.open_read_stream();
            },

            StreamOption::TcpServer => {
                result = self.tcp_server.open_read_stream();
            },

            StreamOption::Udp => {
                result = self.udp.open_read_stream();
            },
        }

        result
    }

    pub fn open_output(&self, output_option: &StreamOption) -> Result<WriteStream, String> {
        let result: Result<WriteStream, String>;

        match output_option {
            StreamOption::File => {
                result = self.file.open_write_stream();
            },

            StreamOption::TcpClient => {
                result = self.tcp_client.open_write_stream();
            },

            StreamOption::TcpServer => {
                result = self.tcp_server.open_write_stream();
            },

            StreamOption::Udp => {
                result = self.udp.open_write_stream();
            },
        }

        result
    }
}


/// The stream option identifies the desired stream type for reading or writing
#[derive(FromPrimitive, Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum StreamOption {
    /// The stream is a file
    File = 1,
    /// The stream is a TCP client with a given port
    TcpClient = 2,
    /// The stream is a TCP server with a given port
    TcpServer = 3,
    /// The stream is a UDP socket with a given port
    Udp = 4,
}

/* Input Streams */
/// The file settings are everything needed to open and read from a file as an input or output
/// stream
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileSettings {
    pub file_name: String,
}

impl Default for FileSettings {
    fn default() -> Self {
        FileSettings { file_name: "data.bin".to_string() }
    }
}

impl fmt::Display for FileSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file:{}", self.file_name)
    }
}

impl FromStr for FileSettings {
    type Err = StreamSettingsParseError;
    fn from_str(s: &str) -> Result<FileSettings, StreamSettingsParseError> {
        let prefix = "file:";
        if s.starts_with(prefix) {
            Ok(FileSettings { file_name: s[prefix.len()..].to_string() })
        } else {
            Err(StreamSettingsParseError(()))
        }
    }
}

impl FileSettings {
    pub fn open_read_stream(&self) -> Result<ReadStream, String> {
        let result = File::open(self.file_name.clone())
                       .map(|file| ReadStream::File(BufReader::new(file)))
                       .map_err(|err| format!("File open error for reading: {}", err));

        return result;
    }

    pub fn open_write_stream(&self) -> Result<WriteStream, String> {
        let result = File::create(self.file_name.clone())
                        .map(|outfile| WriteStream::File(outfile))
                        .map_err(|err| format!("File open error for writing: {}", err));

        return result;
    }
}

/// The tcp client settings are everything needed to open and read from a tcp socket as an input or output
/// stream as a tcp client
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TcpClientSettings {
    pub port: u16,
    pub ip: String,
}

impl Default for TcpClientSettings {
    fn default() -> Self {
        TcpClientSettings { port: 8000,
                            ip: "127.0.0.1".to_string()
        }
    }
}

impl fmt::Display for TcpClientSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tcp_client:{}:{}", self.ip, self.port)
    }
}

impl FromStr for TcpClientSettings {
    type Err = StreamSettingsParseError;
    fn from_str(s: &str) -> Result<TcpClientSettings, StreamSettingsParseError> {
        let prefix = "tcp_client:";
        if s.starts_with(prefix) {
            let mut parts = s[prefix.len()..].split(':');
            let addr = parts.next().ok_or(StreamSettingsParseError(()))?;
            let port_str = parts.next().ok_or(StreamSettingsParseError(()))?;
            let port = port_str.parse::<u16>().map_err(|_| StreamSettingsParseError(()))?;
            Ok(TcpClientSettings { ip: addr.to_string(), port: port })
        } else {
            Err(StreamSettingsParseError(()))
        }
    }
}

impl TcpClientSettings {
    pub fn open_read_stream(&self) -> Result<ReadStream, String> {
        let addr = SocketAddrV4::new(self.ip.parse().unwrap(),
                                     self.port);
        let result = TcpStream::connect(&addr)
                       .map(|sock| ReadStream::Tcp(sock))
                       .map_err(|err| format!("TCP Client Open Error: {}", err));

        return result;
    }

    pub fn open_write_stream(&self) -> Result<WriteStream, String> {
        let addr = SocketAddrV4::new(self.ip.parse().unwrap(),
                                     self.port);

        let result = TcpStream::connect(&addr)
                       .map(|sock| WriteStream::Tcp(sock))
                       .map_err(|err| format!("TCP Client Open Error: {}", err));

        return result;
    }
}

/// The tcp server settings are everything needed to open and read from a tcp socket as an input or output
/// stream as a tcp server
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TcpServerSettings {
    pub port: u16,
    pub ip: String,
}

impl Default for TcpServerSettings {
    fn default() -> Self {
        TcpServerSettings { port: 8000,
                            ip: "127.0.0.1".to_string()
        }
    }
}

impl fmt::Display for TcpServerSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tcp_server:{}:{}", self.ip, self.port)
    }
}

impl FromStr for TcpServerSettings {
    type Err = StreamSettingsParseError;
    fn from_str(s: &str) -> Result<TcpServerSettings, StreamSettingsParseError> {
        let prefix = "tcp_client:";
        if s.starts_with(prefix) {
            let mut parts = s[prefix.len()..].split(':');
            let addr = parts.next().ok_or(StreamSettingsParseError(()))?;
            let port_str = parts.next().ok_or(StreamSettingsParseError(()))?;
            let port = port_str.parse::<u16>().map_err(|_| StreamSettingsParseError(()))?;
            Ok(TcpServerSettings { ip: addr.to_string(), port: port })
        } else {
            Err(StreamSettingsParseError(()))
        }
    }
}

impl TcpServerSettings {
    pub fn open_read_stream(&self) -> Result<ReadStream, String> {
        let addr = SocketAddrV4::new(self.ip.parse().unwrap(), self.port);
        let listener = TcpListener::bind(&addr).unwrap();
        let (sock, _) = listener.accept().map_err(|err| format!("TCP Server Open Error: {}", err))?;
        return Ok(ReadStream::Tcp(sock));
    }

    pub fn open_write_stream(&self) -> Result<WriteStream, String> {
        let addr = SocketAddrV4::new(self.ip.parse().unwrap(), self.port);
        let listener = TcpListener::bind(&addr).unwrap();

        let result = listener.accept()
                             .map(|(sock, _)| WriteStream::Tcp(sock))
                             .map_err(|err| format!("TCP Server Open Error: {}", err));

        return result;
    }
}

/// The udp settings are everything needed to open a UDP socket and use it as an input or output
/// stream
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UdpSettings {
    pub port: u16,
    pub ip: String,
}

impl Default for UdpSettings {
    fn default() -> Self {
        UdpSettings { port: 8001,
                      ip: "127.0.0.1".to_string()
        }
    }
}

impl fmt::Display for UdpSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "udp:{}:{}", self.ip, self.port)
    }
}

impl FromStr for UdpSettings {
    type Err = StreamSettingsParseError;
    fn from_str(s: &str) -> Result<UdpSettings, StreamSettingsParseError> {
        let prefix = "tcp_client:";
        if s.starts_with(prefix) {
            let mut parts = s[prefix.len()..].split(':');
            let addr = parts.next().ok_or(StreamSettingsParseError(()))?;
            let port_str = parts.next().ok_or(StreamSettingsParseError(()))?;
            let port = port_str.parse::<u16>().map_err(|_| StreamSettingsParseError(()))?;
            Ok(UdpSettings { ip: addr.to_string(), port: port })
        } else {
            Err(StreamSettingsParseError(()))
        }
    }
}


impl UdpSettings {
    pub fn open_read_stream(&self) -> Result<ReadStream, String> {
        let sock = UdpSocket::bind("0.0.0.0:0").map_err(|_err| "Couldn't bind to udp address/port")?;
        return Ok(ReadStream::Udp(sock));
    }

    pub fn open_write_stream(&self) -> Result<WriteStream, String> {
        let result;

        match self.ip.parse() {
            Ok(ip_addr) => {
                let addr = SocketAddrV4::new(ip_addr, self.port);

                result = UdpSocket::bind("0.0.0.0:0")
                         .map(|udp_sock| WriteStream::Udp((udp_sock, addr)))
                         .map_err(|err| format!("Could not open UDP socket for writing: {}", err));
            },

            Err(e) => {
                result = Err(format!("Could not parse ip ({}): {}", self.ip, e));
            },
        }

        return result;
    }
}


/* Input/Output Streams */
/// A read stream is a source of bytes.
///
/// This enum allows a caller to return a read stream without using
/// trait objects.
#[derive(Debug)]
pub enum ReadStream {
    File(BufReader<File>),
    Udp(UdpSocket),
    Tcp(TcpStream),
    Null,
}

impl Default for ReadStream {
    fn default() -> ReadStream {
        return ReadStream::Null;
    }
}

impl ReadStream {
    pub fn stream_read(&mut self,
                       bytes: &mut BytesMut,
                       num_bytes: usize) -> Result<usize, String> {

        let result: Result<usize, String>;

        match self {
            ReadStream::File(ref mut file) => {
                result = file.read_bytes(bytes, num_bytes);
            },

            ReadStream::Udp(udp_sock) => {
                // for UDP we just read a message
                result = udp_sock.read_bytes(bytes, num_bytes);
            },

            ReadStream::Tcp(tcp_stream) => {
                result = tcp_stream.read_bytes(bytes, num_bytes);
            },

            ReadStream::Null => {
                // TODO is this an error, or should it just always return no bytes?
                result = Err("Reading a Null Stream! This should not happen!".to_string());
            },
        }

        result
    }
}


/// A write stream, wrapped in an enum to allow multiple write streams to be
/// returned from functions while still allowing the calling function to 
/// defer the choice of stream.
///
/// This is the closed, static way to do this- the open, dynamic way would
/// be trait objects.
#[derive(Debug)]
pub enum WriteStream {
    File(File),
    Udp((UdpSocket, SocketAddrV4)),
    Tcp(TcpStream),
    Null,
}

impl WriteStream {
    pub fn stream_send(&mut self, packet: &Vec<u8>) -> Result<usize, String> {
        let result;

        match self {
            WriteStream::File(file) => {
                result = file.write_bytes(&packet);
            },

            WriteStream::Udp(udp_stream) => {
                result = udp_stream.write_bytes(&packet);
            },

            WriteStream::Tcp(tcp_stream) => {
                result = tcp_stream.write_bytes(&packet);
            },

            WriteStream::Null => {
                result = Ok(0);
            },
        }

        return result;
    }
}

impl Default for WriteStream {
    fn default() -> WriteStream {
        return WriteStream::Null;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamSettingsParseError(());

impl fmt::Display for StreamSettingsParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

impl Error for StreamSettingsParseError {
    fn description(&self) -> &str {
        "error parsing stream settings"
    }
}
