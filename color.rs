pub fn red     (s: &str) -> ~str { format!("\x1b[31m{}\x1b[0m", s) }
pub fn green   (s: &str) -> ~str { format!("\x1b[32m{}\x1b[0m", s) }
pub fn yellow  (s: &str) -> ~str { format!("\x1b[33m{}\x1b[0m", s) }
pub fn blue    (s: &str) -> ~str { format!("\x1b[34m{}\x1b[0m", s) }
pub fn magenta (s: &str) -> ~str { format!("\x1b[35m{}\x1b[0m", s) }
pub fn cyan    (s: &str) -> ~str { format!("\x1b[36m{}\x1b[0m", s) }
pub fn bold    (s: &str) -> ~str { format!("\x1b[;1m{}\x1b[0m", s) }
