extern crate collections;

use self::collections::hashmap::{HashMap};

#[deriving(Clone)]
#[deriving(Show)]
pub struct Packet {
  pub command: StrBuf,
  pub param: Option<StrBuf>,
  pub args: HashMap<StrBuf, StrBuf>,
  pub body: Option<StrBuf>
}

impl Packet {
  pub fn ok(&self) -> bool {
    match self.args.find(&StrBuf::from_str("e")) {
      Some(x) => *x == StrBuf::from_str("ok"),
      None => true
    }
  }

  pub fn param(&self) -> StrBuf {
    self.param.clone().unwrap()
  }

  pub fn body(&self) -> StrBuf {
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

  pub fn parse(pkt: StrBuf) -> ~Packet {
    let chunks = split(pkt, "\n\n");
    let chunknum = chunks.len();
    let (chunk_head, body) = unconsf(chunks, |n| n, |m| m.connect("\n\n"));
    let metadata = split(chunk_head, "\n");
    let (head, meta_tail) = uncons(metadata);
    let mut pktHead:StrBuf;
    let mut pktParam:Option<StrBuf> = None;
    let mut pktArgs:HashMap<StrBuf, StrBuf> = HashMap::with_capacity(4);
    match split(head, " ").as_slice() {
      [] => unreachable!(),
      [ref x] => pktHead = x.to_strbuf(),
      [ref x,ref y,..] => { pktHead = x.to_strbuf(); pktParam = Some(y.to_strbuf()) }
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
        Some(body.to_strbuf())
      }
    }
  }
}

fn split(st: StrBuf, sep: &'static str) -> Vec<StrBuf> {
  st.as_slice().split_str(sep).map(|x|StrBuf::from_str(x)).collect()
}

fn splitn_char(st: StrBuf, sep: char, count: uint) -> Vec<StrBuf> {
  st.as_slice().splitn(sep, count).map(|x|StrBuf::from_str(x)).collect()
}

fn uncons(m: Vec<StrBuf>) -> (StrBuf, Vec<StrBuf>) {
  let mut m = m;
  match m.shift() {
    Some(s) => (s, m),
    None => fail!("empty vector given to uncons")
  }
}

fn unconsf<a,b>(m: Vec<StrBuf>, h: |StrBuf| -> a, t: |Vec<StrBuf>| -> b) -> (a,b) {
  let (head, tail) = uncons(m);
  (h(head), t(tail))
}
