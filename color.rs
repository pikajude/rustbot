pub fn red     (s: &str) -> ~str { fmt!("\x1b[31m%s\x1b[0m", s) }
pub fn green   (s: &str) -> ~str { fmt!("\x1b[32m%s\x1b[0m", s) }
pub fn yellow  (s: &str) -> ~str { fmt!("\x1b[33m%s\x1b[0m", s) }
pub fn blue    (s: &str) -> ~str { fmt!("\x1b[34m%s\x1b[0m", s) }
pub fn magenta (s: &str) -> ~str { fmt!("\x1b[35m%s\x1b[0m", s) }
pub fn cyan    (s: &str) -> ~str { fmt!("\x1b[36m%s\x1b[0m", s) }
pub fn bold    (s: &str) -> ~str { fmt!("\x1b[;1m%s\x1b[0m", s) }
