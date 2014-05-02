pub fn red     (s: StrBuf) -> StrBuf { (format!("\x1b[31m{}\x1b[0m", s)).to_strbuf() }
pub fn green   (s: StrBuf) -> StrBuf { (format!("\x1b[32m{}\x1b[0m", s)).to_strbuf() }
pub fn yellow  (s: StrBuf) -> StrBuf { (format!("\x1b[33m{}\x1b[0m", s)).to_strbuf() }
pub fn blue    (s: StrBuf) -> StrBuf { (format!("\x1b[34m{}\x1b[0m", s)).to_strbuf() }
pub fn magenta (s: StrBuf) -> StrBuf { (format!("\x1b[35m{}\x1b[0m", s)).to_strbuf() }
pub fn cyan    (s: StrBuf) -> StrBuf { (format!("\x1b[36m{}\x1b[0m", s)).to_strbuf() }
pub fn bold    (s: StrBuf) -> StrBuf { (format!("\x1b[;1m{}\x1b[0m", s)).to_strbuf() }
