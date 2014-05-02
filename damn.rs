use color::red;
use packet::{Packet};
use std::io::net::ip::{SocketAddr};
use std::io::net::tcp::{TcpStream};
use std::io::{Reader,Writer};
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
      (self.f)(damn, pkt).map(|res| { (self.trigger.to_owned(), res) })
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
    match self.sock.write(string.as_bytes()) {
      Ok(()) => match self.sock.write(&[10, 0]) {
        Ok(()) => (),
        Err(e) => fail!("{}", e)
      },
      Err(e) => fail!("{}", e)
    }
  }

  pub fn read(&mut self) -> ~str {
    let mut buf = ~[0, .. 8192];
    let len = self.sock.read(buf).unwrap();
    match str::from_utf8(buf.slice_to(len)) {
      Some(s) => s.to_owned(),
      None => fail!("Decoding error")
    }
  }

  pub fn make() -> Option<~Damn> {
    let addr = SocketAddr {
      ip: from_str("199.15.160.100").unwrap(),
      port: 3900
    };
    match TcpStream::connect(addr) {
      Ok(sk) => Some(~Damn { sock: sk }),
      Err(_) => None
    }
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
    let mut results = ~[];
    for hook in self.hooks.iter() {
      results.push(hook.execute(self.damn, pkt))
    }
    results
  }
}

pub type Callback = fn(&mut Damn, &Packet) -> Option<~str>;

fn login(d: &mut Damn, _p: &Packet) -> Option<~str> {
  d.write(~"login formerly-aughters\npk=5503203bc1ded20fed5f669200ea39f6");
  None
}

fn login_callback(d: &mut Damn, p: &Packet) -> Option<~str> {
  if p.ok() {
    println!("{}", red("logged in successfully"));
    None
  } else {
    Some(~"login failure")
  }
}

fn main() {
  let mut bot = Bot::make();
  bot.hook(~"dAmnServer", login);
  bot.hook(~"login", login_callback);
  bot.write(~"dAmnClient 0.3\nagent=rustbot 0.1");
  loop {
    let pkt = bot.read_pkt();
    println!("{}", pkt);
    bot.react(pkt);
  }
}
