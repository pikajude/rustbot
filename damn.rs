extern mod extra;

use std::rt::io::net::ip::{Ipv4};
use std::rt::io::net::tcp::*;
use std::rt::io::{Reader,Writer};
use std::str;

pub struct Damn {
  sock: TcpStream
}

impl Damn {
  pub fn write(&mut self, string: ~str) {
    self.sock.write(string.as_bytes());
    self.sock.write(&[10, 0])
  }

  pub fn read(&mut self) -> ~str {
    let mut buf = ~[0, .. 8192];
    let len = self.sock.read(buf).unwrap();
    str::from_bytes(buf.slice_to(len))
  }

  pub fn new(stream: TcpStream) -> Damn {
    Damn { sock: stream }
  }

  pub fn make() -> Option<~Damn> {
    do TcpStream::connect(Ipv4(199, 15, 160, 100, 3900)).map_consume |s| { ~Damn::new(s) }
  }
}
