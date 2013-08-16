extern mod extra;

use color::*;
use packet::{Packet};
use std::rt::io::net::ip::{IpAddr,SocketAddr};
use std::rt::io::net::tcp::{TcpStream};
use std::rt::io::{Reader,Writer};
use std::str;

mod color;
mod packet;

pub struct Bot {
  hooks: ~[~Hook],
  damn: ~Damn
}

pub struct Hook {
  trigger: ~str,
  f: Callback
}

impl Clone for Hook {
  fn clone(&self) -> Hook {
    match *self {
      Hook{trigger: ref t, f: f} => {
        let f_:Callback = f;
        Hook{trigger: t.clone(), f: f_}
      }
    }
  }
}

impl Hook {
  pub fn execute(&self, damn: &mut Damn, pkt: &Packet) -> Option<(~str, ~str)> {
    if self.trigger == pkt.command {
      do (self.f)(damn, pkt).map_move |res| { (self.trigger.to_owned(), res) }
    } else {
      None
    }
  }
}

pub struct Damn {
  sock: TcpStream
}

impl Damn {
  pub fn write(&mut self, string: ~str) {
    self.sock.write(string.as_bytes());
    self.sock.write(&[10, 0])
  }

  pub fn read(&mut self) -> ~str {
    let mut buf = ~[0, .. 8192];
    let len = self.sock.read(buf).unwrap();
    str::from_bytes(buf.slice_to(len))
  }

  pub fn make() -> Option<~Damn> {
    let addr = SocketAddr {
      ip: FromStr::from_str::<IpAddr>("199.15.160.100").unwrap(),
      port: 3900
    };
    do TcpStream::connect(addr).map_move |s| { ~Damn { sock: s } }
  }
}

impl Bot {
  pub fn make() -> ~Bot {
    let d = Damn::make().unwrap();
    ~Bot {
      hooks: ~[],
      damn: d
    }
  }

  pub fn write(&mut self, pkt: ~str) {
    self.damn.write(pkt);
  }

  pub fn read_pkt(&mut self) -> ~Packet {
    Packet::parse(self.damn.read())
  }

  pub fn hook(&mut self, trigger: ~str, f: Callback) {
    self.hooks.push(~Hook { trigger: trigger, f: f })
  }

  pub fn react(&mut self, pkt: &Packet) -> ~[Option<(~str,~str)>] {
    do self.hooks.iter().map |hook| {
      hook.execute(self.damn, pkt)
    }.collect()
  }
}

type Callback = extern fn(&mut Damn, &Packet) -> Option<~str>;

fn login(d: &mut Damn, p: &Packet) -> Option<~str> {
  d.write(~"login participle\npk=53d8fc2eda3a1719737e031d938b3de8");
  None
}

fn main() {
  let mut bot = Bot::make();
  bot.hook(~"dAmnServer", login);
  bot.write(~"dAmnClient 0.3\nagent=rustbot 0.1");
  loop {
    let pkt = bot.read_pkt();
    printfln!("%?", pkt);
    bot.react(pkt);
  }
}
