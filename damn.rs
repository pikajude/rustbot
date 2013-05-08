extern mod std;

use core::result::*;
use std::net_tcp::*;

pub struct Damn {
  sock: std::net_tcp::TcpSocketBuf
}

impl Damn {
  pub fn write(&self, string: ~str) {
    self.sock.write_str(string);
    self.sock.write_str("\n\x00")
  }

  pub fn read(&self) -> ~str {
    self.sock.read_c_str()
  }
}

pub fn make_damn() -> result::Result<~Damn, TcpConnectErrData> {
  let curTask = &std::uv::global_loop::get(),
      ips = unwrap(std::net_ip::get_addr("chat.deviantart.com", curTask)),
      wrapped_sock = connect(copy ips[0], 3900, curTask);
  match wrapped_sock {
    Ok(s) => Ok(~Damn { sock: socket_buf(s) }),
    Err(v) => Err(v)
  }
}
