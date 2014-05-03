extern crate collections;

use self::collections::hashmap::{HashMap};

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Show)]
pub enum PacketType {
  DamnServer,
  Login
}

pub type ByteString = Vec<u8>;
pub type Text = StrBuf;

#[deriving(Clone)]
#[deriving(Show)]
pub struct Packet {
  pub command: PacketType,
  pub param: Option<Text>,
  pub args: HashMap<Text, Text>,
  pub body: Option<ByteString>
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

  pub fn body(&self) -> Vec<u8> {
    self.body.clone().unwrap()
  }

  /// Tries to parse `body` as a subpacket.
  pub fn subpacket(&self) -> Option<Packet> {
    self.body.clone().map(|bod| { Packet::parse(bod.as_slice()) })
  }

  /// Tries to parse `body` as a subpacket, consuming `self` in the process.
  pub fn subpacket_move(~self) -> Option<Packet> {
    self.body.map(|bod| { Packet::parse(bod.as_slice()) })
  }

  fn cmd_to_type(x: &[u8]) -> PacketType {
    match x {
      [100, 65, 109, 110, 83, 101, 114, 118, 101, 114] => DamnServer,
      [108, 111, 103, 105, 110] => Login,
      e => fail!("unknown type: {}", e)
    }
  }

  pub fn parse(pkt: &[u8]) -> Packet {
    let chunks:Vec<ByteString> = split_vec(pkt, [10, 10]);
    let chunknum = chunks.len();
    let (chunk_head, body) = unconsf(chunks, |n| n, |m| connect_vec(m, [10, 10]));
    let metadata:Vec<&[u8]> = chunk_head.as_slice().split(|x| *x == 10).collect();
    let (head, meta_tail):(&[u8], Vec<&[u8]>) = uncons(metadata);
    let mut pktHead:PacketType;
    let mut pktParam:Option<StrBuf> = None;
    let mut pktArgs:HashMap<Text, Text> = HashMap::with_capacity(4);
    let heads:Vec<&[u8]> = head.as_slice().split(|x:&u8| *x == 32).collect();
    match heads.as_slice() {
      [] => unreachable!(),
      [x] => pktHead = Packet::cmd_to_type(x),
      [x,y,..] => {
        pktHead = Packet::cmd_to_type(x);
        pktParam = Some(StrBuf::from_utf8(Vec::from_slice(y)).unwrap())
      }
    }
    for x in meta_tail.move_iter() {
      let mut pair:Vec<&[u8]> = x.as_slice().splitn(1, |x:&u8| *x == 61).collect();
      if pair.len() == 2 {
        let key = pair.shift(); // determinism!!!
        let value = pair.shift();
        match (key, value) {
          (Some(k), Some(v)) => pktArgs.insert(StrBuf::from_utf8(Vec::from_slice(k)).unwrap(), StrBuf::from_utf8(Vec::from_slice(v)).unwrap()),
          _ => fail!("One of key or value was not found!")
        }
      } else {
        false
      };
    }
    Packet {
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

fn split_vec<T:Eq + Clone>(st: &[T], sep: &[T]) -> Vec<Vec<T>> {
  let len = st.len();
  let mut current = Vec::new();
  let mut results = Vec::new();
  let mut l = 0;
  while l < len {
    if st.slice_from(l).starts_with(sep) {
      results.push(current.clone());
      l += sep.len();
      current = Vec::new();
    } else {
      current.push(st[l].clone());
      l += 1;
    }
  }
  results.push(current.clone());
  results
}

fn connect_vec<T: Clone>(st: Vec<Vec<T>>, sep: &[T]) -> Vec<T> {
  let mut out = Vec::new();
  out = out.append(st.get(0).as_slice());
  for l in range(1, st.len()) {
    out = out.append(sep);
    out = out.append(st.get(l).as_slice());
  }
  out
}

fn uncons<V>(m: Vec<V>) -> (V, Vec<V>) {
  let mut m = m;
  match m.shift() {
    Some(s) => (s, m),
    None => fail!("empty vector given to uncons")
  }
}

fn unconsf<a,b,V>(m: Vec<V>, h: |V| -> a, t: |Vec<V>| -> b) -> (a,b) {
  let (head, tail) = uncons(m);
  (h(head), t(tail))
}
