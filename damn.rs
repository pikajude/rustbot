extern mod extra;

use std::rt::io::net::tcp::*;
use std::rt::io::net::ip::{Ipv4};
use std::rt::io::{Reader,Writer};
use std::result;
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
    let buf = &mut [];
    self.sock.read(buf);
    str::from_bytes(buf)
  }
}

fn main() {
  println(fmt!("%s", Ipv4(194, 199, 94, 211, 80).to_str()));
}

pub fn make_damn() -> result::Result<~Damn, ~str> {
  Err(~"fuck")
}

// pub fn make_damn() -> result::Result<~Damn, TcpConnectErrData> {
//   let curTask = &std::uv::global_loop::get();
//   let ips = unwrap(std::net_ip::get_addr("chat.deviantart.com", curTask));
//   let wrapped_sock = connect(ips[0], 3900, curTask);
//   match wrapped_sock {
//     Ok(s) => Ok(~Damn { sock: socket_buf(s) }),
//     Err(v) => Err(v)
//   }
// }
