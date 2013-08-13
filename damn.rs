extern mod extra;

use extra::future;
use std::cell::{Cell};
use std::hashmap::{HashMap};
// use std::rand::*;
use std::comm::{SharedChan,Port};
use std::rt::io::net::ip::{IpAddr,SocketAddr};
use std::rt::io::net::tcp::{TcpStream};
use std::rt::io::{Reader,Writer};
use std::select;
use std::str;

use packet::{Packet};

mod packet;

pub struct Bot {
  hooks: ~[~Hook],
  in_pipe: SharedChan<~str>,
  sock_in: SharedChan<~str>
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
  pub fn execute(&self, damn: SharedChan<~str>, pkt: ~Packet) -> Option<(~str, ~str)> {
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
    // let d = Damn::make().unwrap();
    let (out, in_) = stream();
    let (sock_out, sock_in) = stream(); // d.sock;
    let (shared_in, shared_sock_in) = (SharedChan::new(in_), SharedChan::new(sock_in));
    let output_cell = Cell::new(~[sock_out, out]);
    do spawn {
      let mut outs:~[Port<~str>] = output_cell.take();
      loop {
        let ix = select::select(outs);
        if outs[ix].peek() {
          println(outs[ix].recv());
        } else {
          printfln!("Pipe %u reported true but has no data.", ix);
        }
      }
    }
    ~Bot {
      hooks: ~[],
      in_pipe: shared_in,
      sock_in: shared_sock_in
    }
  }

  pub fn write(&mut self, pkt: ~str) {
    self.in_pipe.send(pkt);
  }

  pub fn hook(&mut self, trigger: ~str, f: Callback) {
    self.hooks.push(~Hook { trigger: trigger, f: f })
  }

  pub fn react(&mut self, pkt: ~Packet) -> ~[Option<(~str,~str)>] {
    let pkt_cell = Cell::new(pkt.clone());
    let pipe_cell = Cell::new(self.sock_in.clone());
    let futures:~[future::Future<Option<(~str,~str)>>] = do self.hooks.iter().map |hook| {
      let mypkt = pkt_cell.take();
      pkt_cell.put_back(mypkt.clone());
      let mypktcell = Cell::new(mypkt);
      let mypipe = pipe_cell.take();
      pipe_cell.put_back(mypipe.clone());
      let mypipecell = Cell::new(mypipe);
      let h = hook.clone();
      do extra::future::spawn {
        h.execute(mypipecell.take(), mypktcell.take())
      }
    }.collect();
    futures.move_iter().map(|x| { let mut y = x; y.get() }).collect()
  }
}

type Callback = extern fn(SharedChan<~str>, ~Packet) -> Option<~str>;

fn reactor(damn: SharedChan<~str>, pkt: ~Packet) -> Option<~str> {
  damn.send(fmt!("From handle pipe: %?", pkt));
  None
}

fn main() {
  let mut bot = Bot::make();
  let mut args = HashMap::new();
  args.insert(~"a", ~"b");
  args.insert(~"c", ~"d");
  bot.hook(~"dAmnServer", reactor);
  bot.hook(~"dAmnServer", reactor);
  bot.hook(~"dAmnServer", reactor);
  bot.hook(~"dAmnServer", reactor);
  bot.hook(~"dAmnServer", reactor);
  bot.react(~Packet {
    command: ~"dAmnServer",
    param: Some(~"0.3"),
    args: args,
    body: None
  });
  bot.in_pipe.send(~"From sock pipe: ello m8s");
  std::io::stdin().read_line();
}
