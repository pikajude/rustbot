extern crate collections;

use self::collections::hashmap::{HashMap};

#[deriving(Clone)]
#[deriving(Show)]
pub struct Packet {
  command: ~str,
  param: Option<~str>,
  args: HashMap<~str, ~str>,
  body: Option<~str>
}

impl Packet {
  pub fn ok(&self) -> bool {
    match self.args.find(&~"e") {
      Some(x) => x.eq(&~"ok"),
      None => true
    }
  }

  pub fn param(&self) -> ~str {
    self.param.clone().unwrap()
  }

  pub fn body(&self) -> ~str {
    self.body.clone().unwrap()
  }

  pub fn subpacket(&self) -> Option<~Packet> {
    self.body.clone().map(|bod| { Packet::parse(bod) })
  }

  pub fn subpacket_move(~self) -> Option<~Packet> {
    self.body.map(|bod| { Packet::parse(bod) })
  }

  pub fn subpacket_(&self) -> ~Packet {
    Packet::parse(self.body.clone().unwrap())
  }

  pub fn subpacket_move_(~self) -> ~Packet {
    Packet::parse(self.body.unwrap())
  }

  pub fn parse(pkt: ~str) -> ~Packet {
    let chunks = split(pkt, "\n\n");
    let chunknum = chunks.len();
    let (chunk_head, body) = unconsf(chunks, |n| n, |m| m.connect("\n\n"));
    let metadata = split(chunk_head, "\n");
    let (head, meta_tail) = uncons(metadata);
    let mut pktHead:~str;
    let mut pktParam:Option<~str> = None;
    let mut pktArgs:HashMap<~str, ~str> = HashMap::with_capacity(4);
    match split(head, " ").as_slice() {
      [] => unreachable!(),
      [ref x] => pktHead = x.to_owned(),
      [ref x,ref y,..] => { pktHead = x.to_owned(); pktParam = Some(y.to_owned()) }
    }
    for x in meta_tail.move_iter() {
      let mut pair = splitn_char(x, '=', 1);
      if pair.len() == 2 {
        let key = pair.shift(); // determinism!!!
        let value = pair.shift();
        match (key, value) {
          (Some(k), Some(v)) => pktArgs.insert(k, v),
          _ => fail!("One of key or value was not found!")
        }
      } else {
        false
      };
    }
    ~Packet {
      command: pktHead,
      param: pktParam,
      args: pktArgs,
      body: if chunknum == 1 {
        None
      } else {
        Some(body)
      }
    }
  }
}

fn split(st: ~str, sep: &'static str) -> ~[~str] {
  st.split_str(sep).map(|x| x.to_owned()).collect()
}

fn splitn_char(st: ~str, sep: char, count: uint) -> ~[~str] {
  st.splitn(sep, count).map(|x| x.to_owned()).collect()
}

fn uncons(m: ~[~str]) -> (~str, ~[~str]) {
  let mut m = m;
  match m.shift() {
    Some(s) => (s, m),
    None => fail!("empty vector given to uncons")
  }
}

fn unconsf<a,b>(m: ~[~str], h: |~str| -> a, t: |~[~str]| -> b) -> (a,b) {
  let (head, tail) = uncons(m);
  (h(head), t(tail))
}
