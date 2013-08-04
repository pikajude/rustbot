extern mod extra;

use damn::Damn;
use packet::Packet;
use protocol::*;
use std::os;

mod color;
mod damn;
mod packet;
mod protocol;

fn main() {
  let mut damn = Damn::make().unwrap();
  damn.write(~"dAmnClient 0.3\nagent=rustbot 0.1");
  loop {
    let pk = Packet::parse(damn.read());
    let failure = match pk.command {
      ~"dAmnServer" => r_dAmnServer(damn, pk),
      ~"login" => r_login(damn, pk),
      _ => {
        println(fmt!("%?", pk));
        Some(fmt!("Unhandled packet %s", pk.command))
      }
    };
    match failure {
      Some(e) => {
        printfln!("Something went wrong! %s", e);
        os::set_exit_status(1);
        break
      }
      _ => {}
    }
  }
}
