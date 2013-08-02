extern mod std;

use damn::Damn;

mod damn;
mod packet;
// mod protocol;

fn react(damn: ~Damn) {
  let mut damn = damn;
  damn.write(~"dAmnClient 0.3\nagent=foobar");
  loop {
    let pk = packet::parse(damn.read());
    match pk.command {
      ~"dAmnServer" => {
        damn.write(~"login aughters\npk=lol");
      }
      _ => {}
    }
    println(fmt!("%?", pk));
  }
}

fn main() {
  match damn::make_damn() {
    Ok(v) => react(v),
    Err(k) => println(k)
  }
}
