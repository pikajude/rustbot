extern mod std;

mod damn;
mod packet;
mod protocol;

fn respond(sock: std::net_tcp::TcpSocketBuf) {
  let pk = packet::parse(sock.read_c_str());
  match pk.command {
    ~"dAmnServer" => {
      sock.write_str("login aughters\npk=lol\n\x00");
    }
    _ => {}
  }
  io::println(fmt!("%?", pk));
}

fn main() {
  match damn::make_damn() {
    Ok(v) => {
      v.sock.write_str("dAmnClient 0.3\nagent=foobar\n\x00");
      loop {
        respond(v.sock);
      }
    },
    Err(k) => io::println(fmt!("%?", k))
  }
}
