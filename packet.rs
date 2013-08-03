use std::hashmap::*;

pub struct Packet {
  command: ~str,
  param: Option<~str>,
  args: HashMap<~str, ~str>,
  body: Option<~str>
}

impl Packet {
  pub fn ok(&self) -> bool {
    match self.args.find(&~"e") {
      Some(&~"ok") => true,
      None => true,
      _ => false
    }
  }

  pub fn subpacket(&self) -> Option<~Packet> {
    do self.body.map |bod| { parse(bod.clone()) }
  }

  pub fn subpacket_consume(~self) -> Option<~Packet> {
    do self.body.map_consume |bod| { parse(bod) }
  }

  pub fn subpacket_(&self) -> ~Packet {
    match self.body.clone() {
      None => fail!("Tried to unwrap None"),
      Some(v) => parse(v)
    }
  }

  pub fn subpacket_consume_(~self) -> ~Packet {
    match self.body {
      None => fail!("Tried to unwrap None"),
      Some(v) => parse(v)
    }
  }
}

fn split(st: ~str, sep: &'static str) -> ~[~str] {
  do st.split_str_iter(sep).to_owned_vec().map |x| { x.to_owned() }
}

fn splitn_char(st: ~str, sep: char, count: uint) -> ~[~str] {
  do st.splitn_iter(sep, count).to_owned_vec().map |x| { x.to_owned() }
}

fn uncons(m: ~[~str]) -> (~str, ~[~str]) {
  let mut m = m;
  let h = m.shift();
  (h, m)
}

fn unconsf(m: ~[~str], h: &fn(~str) -> ~str, t: &fn(~[~str]) -> ~str) -> (~str, ~str) {
  let (hh, tt) = uncons(m);
  (h(hh), t(tt))
}

pub fn parse(pkt: ~str) -> ~Packet {
  let chunks = split(pkt, "\n\n");
  let chunknum = chunks.len();
  let (chunk_head, body) = unconsf(chunks, |n| n, |m| m.connect("\n\n"));
  let metadata = split(chunk_head, "\n");
  let (head, meta_tail) = uncons(metadata);
  let mut pktHead:~str;
  let mut pktParam:Option<~str> = None;
  let mut pktArgs:HashMap<~str, ~str> = linear_map_with_capacity(4);
  match split(head, " ") {
    [] => fail!("impossible"),
    [x] => pktHead = x,
    [x,y,.._] => { pktHead = x; pktParam = Some(y) }
  }
  match meta_tail {
    [] => {},
    xs => {
      foreach x in xs.consume_iter() {
        let mut pair = splitn_char(x, '=', 1);
        if pair.len() == 2 {
          let f = pair.shift(); // determinism!!!
          pktArgs.insert(f, pair.shift())
        } else {
          false
        };
      };
    }
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
