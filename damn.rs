#![feature(macro_rules)]

extern crate time;

use color::{green,magenta};
pub use packet::{Packet};
use std::io::net::ip::{SocketAddr};
use std::io::net::tcp::{TcpStream};
use std::io::net::addrinfo::get_host_addresses;
use std::io::{Reader,Writer,IoResult};
use std::strbuf::StrBuf;
use time::{strftime,now};

mod color;
mod packet;

macro_rules! log(
  ($s:expr) => (println!("\x1b[30m{}\x1b[0m {}", time::strftime("%b %d %H:%M:%S", time::now()), $s));
  ($s:expr, $($m:expr),+) => (println!("\x1b[30m{}\x1b[0m {}", time::strftime("%b %d %H:%M:%S", &time::now()), format!($s, $($m),+)))
)

macro_rules! failure(($($s:expr),+) => (Some(format!($($s),+).to_strbuf())))

pub struct Bot {
  hooks: Vec<~Hook>,
  damn: ~Damn
}

pub struct Hook {
  trigger: StrBuf,
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
  pub fn execute(&self, damn: &mut Damn, pkt: &Packet) -> Option<(StrBuf, StrBuf)> {
    if self.trigger == pkt.command {
      (self.f)(damn, pkt).map(|res| { (self.trigger.to_strbuf(), res) })
    } else {
      None
    }
  }
}

pub struct Damn {
  sock: TcpStream
}

impl Damn {
  pub fn write<T: Str>(&mut self, string: T) {
    match self.sock.write(string.to_strbuf().as_bytes()) {
      Ok(()) => match self.sock.write(&[10, 0]) {
        Ok(()) => (),
        Err(e) => fail!("{}", e)
      },
      Err(e) => fail!("{}", e)
    }
  }

  pub fn read(&mut self) -> StrBuf {
    let mut buf = [0, ..8192];
    let len = self.sock.read(buf).unwrap();
    let vbuf = Vec::from_slice(buf);
    match StrBuf::from_utf8(vbuf) {
      Some(s) => s,
      None => fail!("Decoding error")
    }
  }

  pub fn make() -> IoResult<~Damn> {
    get_host_addresses("chat.deviantart.com").and_then(|s| {
      match s.move_iter().next() {
        Some(addr) => TcpStream::connect(SocketAddr{ ip: addr, port: 3900 })
                        .map(|sk| ~Damn { sock: sk }),
        _ => fail!("No IP resolution for chat.deviantart.com!")
      }
    })
  }
}

impl Bot {
  pub fn make() -> ~Bot {
    let d = Damn::make().unwrap();
    ~Bot {
      hooks: Vec::new(),
      damn: d
    }
  }

  pub fn write<T: Str>(&mut self, pkt: T) {
    self.damn.write(pkt);
  }

  pub fn read_pkt(&mut self) -> ~Packet {
    Packet::parse(self.damn.read())
  }

  pub fn hook<T: Str>(&mut self, trigger: T, f: Callback) {
    self.hooks.push(~Hook { trigger: trigger.to_strbuf(), f: f })
  }

  pub fn react(&mut self, pkt: &Packet) -> Vec<Option<(StrBuf,StrBuf)>> {
    let mut results = Vec::new();
    for hook in self.hooks.iter() {
      results.push(hook.execute(self.damn, pkt))
    }
    results
  }
}

pub type Callback = fn(&mut Damn, &Packet) -> Option<StrBuf>;

fn login(d: &mut Damn, p: &Packet) -> Option<StrBuf> {
  log!("Greeting from server: dAmnServer {}", magenta(p.param()));
  d.write("login formerly-aughters\npk=5503203bc1ded20fed5f669200ea39f6");
  None
}

fn login_callback(_d: &mut Damn, p: &Packet) -> Option<StrBuf> {
  if p.ok() {
    log!("Logged in as {}", green(p.param()));
    None
  } else {
    failure!("Login failure: {}", p.args.get(&"e".to_strbuf()))
  }
}

fn main() {
  let mut bot = Bot::make();
  bot.hook("dAmnServer", login);
  bot.hook("login", login_callback);
  bot.write("dAmnClient 0.3\nagent=rustbot 0.1");
  loop {
    let pkt = bot.read_pkt();
    println!("{}", pkt);
    bot.react(pkt);
  }
}
