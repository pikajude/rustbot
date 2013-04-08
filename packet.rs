extern mod core;

use core::hashmap::linear::*;

pub struct Packet {
  command: ~str,
  param: Option<~str>,
  args: LinearMap<~str, ~str>,
  body: Option<~str>
}

fn split(st: ~str, sep: &'static str) -> ~[~str] {
  do vec::build |f| { str::each_split_str(st, sep, |chunk| { f(chunk.to_owned()); true } ) }
}

fn splitn_char(st: &str, sep: char, count: uint) -> ~[~str] {
  do vec::build |f| { str::each_splitn_char(st, sep, count, |chunk| { f(chunk.to_owned()); true }) }
}

pub fn parse(pkt: ~str) -> @Packet {
  let chunks = split(pkt, "\n\n"),
      metadata = split(copy chunks[0], "\n"),
      body = str::connect(vec::tail(chunks), "\n\n"),
      head = copy metadata[0];
  let mut pktHead:~str,
          pktParam:Option<~str> = None,
          pktArgs:LinearMap<~str, ~str> = linear_map_with_capacity(8);
  match split(head, " ") {
    [] => fail!(~"impossible"),
    [x] => pktHead = x,
    [x,y,.._] => { pktHead = x; pktParam = Some(y) }
  }
  match vec::tail(metadata) {
    [] => {},
    xs => {
      let pairs = vec::map(xs, |x| splitn_char(*x, '=', 1));
      vec::each(pairs, |pair| if vec::len(*pair) == 2 {
        pktArgs.insert(copy pair[0], copy pair[1])
      } else {
        false
      });
    }
  }
  @Packet {
    command: pktHead,
    param: pktParam,
    args: pktArgs,
    body: if vec::len(chunks) == 1 {
      None
    } else {
      Some(body)
    }
  }
}
