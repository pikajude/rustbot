extern mod std;

use damn::Damn;

mod damn;
mod packet;
// mod protocol;

fn react(damn: ~Damn) {
  let mut damn = damn;
  damn.write(~"dAmnClient 0.3\nagent=rustbot 0.1");
  loop {
    let pk = packet::parse(damn.read());
    println(fmt!("%?", pk));
    match pk.command {
      ~"dAmnServer" => {
        damn.write(~"login alphacookie\npk=3368b2f8338df98e7cdbe3b9cd8b34ec");
      }
      _ => {}
    }
  }
}

fn main() {
  match damn::make_damn() {
    Some(v) => react(v),
    _ => {}
  }
}
