extern mod std;

mod damn;
mod packet;
mod protocol;

fn respond(pkt: ~str) {
  let pk = packet::parse(pkt);
  match pk.command {
    ~"dAmnServer" => {
      io::println("ello");
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
        respond(v.sock.read_c_str());
      }
    },
    Err(k) => io::println(fmt!("%?", k))
  }
}
