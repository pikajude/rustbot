use damn::{Damn};
use packet::{Packet};
use color::*;
use extra::time;

macro_rules! log (
  ($s:expr) => (
    printfln!("\x1b[30m%s\x1b[0m %s", time::strftime("%b %d %H:%M:%S", ~time::now()), $s)
  );
  ($s:expr, $($m:expr),+) => (
    printfln!("\x1b[30m%s\x1b[0m %s", time::strftime("%b %d %H:%M:%S", ~time::now()), fmt!($s, $($m),+))
  )
)

macro_rules! fail (($($s:expr),+) => (Some(fmt!($($s),+))))

pub fn r_dAmnServer(damn: &mut Damn, packet: &Packet) -> Option<~str> {
  log!("Received greeting from server: dAmnServer %s.", magenta(packet.param()));
  damn.write(~"login alphacookie\npk=3368b2f8338df98e7cdbe3b9cd8b34ec");
  None
}

pub fn r_login(damn: &mut Damn, packet: &Packet) -> Option<~str> {
  if packet.ok() {
    log!("Logged in as %s!", green(packet.param()));
    damn.write(~"join chat:DevelopingDevelopers");
    None
  } else {
    fail!("Auth failure: %s.", packet.args.get(&~"e").to_owned())
  }
}
