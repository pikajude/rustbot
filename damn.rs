#![crate_id="rustbot"]
#![feature(macro_rules)]
#![feature(globs)]

extern crate time;

use color::*;
use packet::*;
use std::io::net::ip::{SocketAddr};
use std::io::net::tcp::{TcpStream};
use std::io::net::addrinfo::get_host_addresses;
use std::io::{Reader,Writer,IoResult};
use std::io::stdio::{stderr};
use std::os;
use std::strbuf::StrBuf;
use time::{strftime,now};

pub mod color;
pub mod packet;

macro_rules! log(
  ($s:expr) => (println!("\x1b[30m{}\x1b[0m {}", time::strftime("%b %d %H:%M:%S", time::now()), $s));
  ($s:expr, $($m:expr),+) => (println!("\x1b[30m{}\x1b[0m {}", time::strftime("%b %d %H:%M:%S", &time::now()), format!($s, $($m),+)))
)

macro_rules! failure(($($s:expr),+) => (Err(format!($($s),+).to_strbuf())))
macro_rules! warn(($($arg:tt),+) => ({
  stderr().write(format!($($arg),+).as_bytes()).and_then(|_| {
    stderr().write(&[10])
  })
}))

pub struct Bot {
  hooks: Vec<Hook>,
  damn: Damn
}

pub struct Hook {
  /// The type of packet this hook reacts to.
  pub trigger: PacketType,
  pub callback: Callback
}

/// NB: clones `trigger` but not `f`.
impl Clone for Hook {
  fn clone(&self) -> Hook {
    match *self {
      Hook{trigger: ref t, callback: f} => {
        let f_:Callback = f;
        Hook{trigger: t.clone(), callback: f_}
      }
    }
  }
}

/// Callbacks may either succeed or provide an error message.
pub type CallbackResult = Result<(), Text>;
pub type Callback = fn(&mut Bot, &Packet) -> CallbackResult;

impl Hook {
  /**
    If the command executes successfully, returns `Ok(())`. Otherwise
    returns `Err("reason for failure")`.
  */
  pub fn execute(&self, bot: &mut Bot, pkt: &Packet) -> CallbackResult {
    if self.trigger == pkt.command {
      (self.callback)(bot, pkt)
    } else {
      Ok(())
    }
  }
}

pub struct Damn {
  sock: TcpStream
}

impl Damn {
  /// Write some `u8` to the socket.
  pub fn write(&mut self, string: &[u8]) {
    match self.sock.write(string) {
      Ok(()) => match self.sock.write(&[10, 0]) {
        Ok(()) => (),
        Err(e) => fail!("{}", e)
      },
      Err(e) => fail!("{}", e)
    }
  }

  fn read(&mut self) -> IoResult<ByteString> {
    let mut buf = [0, ..8192];
    self.sock.read(buf).map(|_|Vec::from_slice(buf))
  }

  /**
    Produces a Damn. The only error that is likely to happen here is an IO
    error (when connecting to dAmn.)

    ```
    match Damn::make() {
      Ok(mut damn) => do_stuff(),
      Err(e) => fail!("{}", e)
    };
    ```
  */
  pub fn make() -> IoResult<Damn> {
    get_host_addresses("chat.deviantart.com").and_then(|s|
      match s.move_iter().next() {
        Some(addr) => TcpStream::connect(SocketAddr { ip: addr, port: 3900 })
                        .map(|sk| Damn { sock: sk }),
        _ => fail!("No IP resolution for chat.deviantart.com!")
      }
    )
  }
}

impl Bot {
  pub fn make() -> IoResult<Bot> {
    Damn::make().map(|d|
      Bot {
        hooks: Vec::new(),
        damn: d
      }
    )
  }

  pub fn write<T: Str>(&mut self, pkt: T) {
    self.damn.write(pkt.to_strbuf().as_bytes());
  }

  pub fn read_pkt(&mut self) -> IoResult<Packet> {
    self.damn.read().map(|x|Packet::parse(x.as_slice()))
  }

  pub fn hook(&mut self, trigger: PacketType, f: Callback) {
    self.hooks.push(Hook { trigger: trigger, callback: f })
  }

  /// Executes all registered hooks for this packet. Returns a list of
  /// execution results.
  pub fn react(&mut self, pkt: &Packet) -> Vec<Result<(), (PacketType, StrBuf)>> {
    let mut results = Vec::new();
    for hook in self.hooks.clone().move_iter() {
      results.push(hook.execute(self, pkt).map_err(|e|(hook.trigger, e)))
    }
    results
  }
}

fn r_dAmnServer(b: &mut Bot, p: &Packet) -> CallbackResult {
  log!("Greeting from server: dAmnServer {}", magenta(p.param()));
  b.write("login formerly-aughters\npk=5503203bc1ded20fed5f669200ea39f6");
  Ok(())
}

fn r_login(_b: &mut Bot, p: &Packet) -> CallbackResult {
  if p.ok() {
    log!("Logged in as {}", green(p.param()));
    Ok(())
  } else {
    failure!("Login failure: {}", p.args.get(&"e".to_strbuf()))
  }
}

pub fn main() {
  match Bot::make() {
    Err(e) => {
      let _res = warn!("Unable to build the bot: {}", e);
      os::set_exit_status(1)
    },
    Ok(mut bot) => {
      bot.hook(DamnServer, r_dAmnServer);
      bot.hook(Login, r_login);
      bot.write("dAmnClient 0.3\nagent=rustbot 0.1");
      loop {
        match bot.read_pkt() {
          Err(e) => fail!("{}", e),
          Ok(pkt) => {
            println!("{}", pkt);
            bot.react(&pkt);
          }
        }
      }
    }
  }
}
