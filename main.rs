extern mod std;

use damn::Damn;

mod damn;
mod packet;
mod protocol;

fn respond(damn: &Damn) {
  let pk = packet::parse(damn.read());
  match pk.command {
    ~"dAmnServer" => {
      damn.write(~"login aughters\npk=lol");
    }
    _ => {}
  }
  io::println(fmt!("%?", pk));
}

fn main() {
  match damn::make_damn() {
    Ok(v) => {
      v.write(~"dAmnClient 0.3\nagent=foobar");
      loop {
        respond(v);
      }
    },
    Err(k) => io::println(fmt!("%?", k))
  }
}
