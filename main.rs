extern mod std;

use damn::Damn;
use packet::Packet;

mod damn;
mod packet;

fn react(damn: ~Damn) {
  let mut damn = damn;
  damn.write(~"dAmnClient 0.3\nagent=rustbot 0.1");
  loop {
    let pk = Packet::parse(damn.read());
    println(fmt!("%?", pk));
    match pk.command {
      ~"dAmnServer" => {
        damn.write(~"login alphacookie\npk=3368b2f8338df98e7cdbe3b9cd8b34ec");
      }
      ~"login" => {
        if pk.ok() {
          println("we are logged in!");
        } else {
          println("we are not logged in!");
        }
      }
      _ => {}
    }
  }
}

fn main() {
  match Damn::make() {
    Some(v) => react(v),
    None => println("Failed to connect to the server. Sorry!")
  }
}
