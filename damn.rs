extern mod std;

use core::result::*;
use std::net_tcp::*;

struct Damn {
  sock: std::net_tcp::TcpSocketBuf
}

pub fn make_damn() -> result::Result<~Damn, TcpConnectErrData> {
  let curTask = &std::uv::global_loop::get(),
      ips = unwrap(std::net_ip::get_addr("chat.deviantart.com", curTask));
  do map(&connect(copy ips[0], 3900, curTask)) |&sk| {
    ~Damn { sock: socket_buf(sk) }
  }
}
