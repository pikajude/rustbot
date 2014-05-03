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
    let chunks = split_vec(pkt, [10, 10]);
    let chunknum = chunks.len();
    let (chunk_head, body) =
      unconsf(
        chunks.as_slice(),
        |n| n,
        |m| connect_vec(m.as_slice(), [10, 10]));
    let metadata:Vec<&[u8]> = chunk_head.as_slice().split(|x:&u8| *x == 10).collect();
    let (head, meta_tail):(&&[u8], &[&[u8]]) = uncons(metadata.as_slice());
    let mut pktHead:PacketType;
    let mut pktParam:Option<StrBuf> = None;
    let mut pktArgs:HashMap<Text, Text> = HashMap::with_capacity(4);
    let heads:Vec<&[u8]> = head.split(|x:&u8| *x == 32).collect();
    match heads.as_slice() {
      [] => unreachable!(),
      [x] => pktHead = Packet::cmd_to_type(x),
      [x,y,..] => {
        pktHead = Packet::cmd_to_type(x);
        pktParam = Some(StrBuf::from_utf8(Vec::from_slice(y)).unwrap())
      }
    }
    for x in meta_tail.iter() {
      let pair:Vec<&[u8]> = x.as_slice().splitn(1, |x:&u8| *x == 61).collect();
      match pair.as_slice() {
        [k, v] => pktArgs.insert(
          StrBuf::from_utf8(Vec::from_slice(k)).unwrap(),
          StrBuf::from_utf8(Vec::from_slice(v)).unwrap()
        ),
        _ => false
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

fn connect_vec<T: Clone>(st: &[Vec<T>], sep: &[T]) -> Vec<T> {
  let mut out = Vec::new();
  out.push_all(st[0].as_slice());
  for l in range(1, st.len()) {
    out.push_all(sep);
    out.push_all(st[l].as_slice());
  }
  out
}

fn uncons<'a, V>(m: &'a [V]) -> (&'a V, &'a [V]) {
  match m.head() {
    Some(h) => (h, m.tail()),
    None => fail!("empty vector given to uncons")
  }
}

fn unconsf<'x,a,b,V>(m: &'x [V], h: |&'x V| -> a, t: |&'x [V]| -> b) -> (a,b) {
  let (head, tail) = uncons(m);
  (h(head), t(tail))
}
