extern mod extra;

use extra::par;
use std::cell::Cell;
use std::comm::*;
use std::pipes::*;
use std::rt::io::net::ip::*;
use std::rt::io::net::tcp::*;
use std::rt::io::{Reader,Writer};
// use std::rt::io::comm_adapters::{ReaderPort,WriterChan};
use std::str;

use packet::{Packet};

mod packet;

pub struct Bot {
  hooks: ~[~Hook],
  in_pipe: SharedChan<~str>,
  sock_in: SharedChan<~str>
}

type Callback = extern fn(SharedChan<~str>, ~Packet) -> Option<~str>;

pub struct Hook {
  trigger: ~str,
  f: Callback
}

impl Clone for Hook {
  pub fn clone(&self) -> Hook {
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
      do (self.f)(damn, pkt).map_consume |res| { (self.trigger.to_owned(), res) }
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
      ip: Ipv4Addr(199, 15, 160, 100),
      port: 3900
    };
    do TcpStream::connect(addr).map_consume |s| { ~Damn { sock: s } }
  }
}

impl Bot {
  pub fn make() -> ~Bot {
    // let mut d = Damn::make().unwrap();
    let (out, in_) = stream();
    let (sock_out, sock_in) = stream(); // (ReaderPort::new(d.sock), WriterChan::new(d.sock));
    let (shared_in, shared_sock_in) = (SharedChan::new(in_), SharedChan::new(sock_in));
    let output_cell = Cell::new(~[out, sock_out]);
    do spawn {
      let mut outs = output_cell.take();
      loop {
        match wait_many(outs) {
          0 => printfln!("A has data!"),
          _ => printfln!("B has data!")
        }
      }
    }
    shared_in.send(~"hello");
    shared_sock_in.send(~"hello");
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

  pub fn react(&mut self, pkt: ~Packet) -> ~[(~str,~str)] {
    par::map(self.hooks, || {
      let pk = Cell::new(pkt.clone());
      let pipe = Cell::new(self.in_pipe.clone());
      |hook| { hook.execute(pipe.take(), pk.take()) }
    }).consume_iter().filter_map(|x|x).collect()
  }
}

fn main() {
  let mut bot = Bot::make();
  println("success!");
}
