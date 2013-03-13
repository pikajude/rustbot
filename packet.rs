extern mod core;

use core::hashmap::linear::*;

pub struct Packet {
  command: ~str,
  param: Option<~str>,
  args: LinearMap<~str, ~str>,
  body: Option<~str>
}

pub fn parse(pkt: ~str) -> @Packet {
  let chunks = str::split_str(pkt, "\n\n"),
      metadata = str::split_str(chunks[0], "\n"),
      body = str::connect(vec::tail(chunks), "\n\n"),
      head = copy metadata[0];
  let mut pktHead:~str,
          pktParam:Option<~str> = None,
          pktArgs:LinearMap<~str, ~str> = linear_map_with_capacity(8);
  match str::split_str(head, " ") {
    [] => fail!(~"impossible"),
    [x] => pktHead = x,
    [x,y,.._] => { pktHead = x; pktParam = Some(y) }
  }
  match vec::tail(metadata) {
    [] => {},
    xs => {
      let pairs = vec::map(xs, |x| str::splitn_char(*x, '=', 1));
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
