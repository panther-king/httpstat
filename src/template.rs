//! Output style template
use std::collections::HashMap;

use progress::Progress;
use self::Process::{Connect, ContentTransfer, DnsLookup, NameLookup, PreTransfer,
                    ServerProcessing, SslHandshake, StartTransfer, TcpConnection, Total};

/// Values for output template
#[derive(Clone)]
pub enum Process {
    Connect,
    ContentTransfer,
    DnsLookup,
    NameLookup,
    PreTransfer,
    ServerProcessing,
    SslHandshake,
    StartTransfer,
    TcpConnection,
    Total,
}

impl Process {
    /// Mappings of template index
    pub fn index(&self) -> String {
        let index = match *self {
            DnsLookup => "a0000",
            TcpConnection => "a0001",
            SslHandshake => "a0002",
            ServerProcessing => "a0003",
            ContentTransfer => "a0004",
            NameLookup => "b0000",
            Connect => "b0001",
            PreTransfer => "b0002",
            StartTransfer => "b0003",
            Total => "b0004",
        };

        index.to_owned()
    }

    /// A format of text-align
    pub fn align(&self, ms: usize) -> String {
        let elapsed = format!("{}ms", ms);
        match *self {
            DnsLookup | TcpConnection | SslHandshake | ServerProcessing | ContentTransfer => {
                format!("{:^7}", elapsed)
            }
            NameLookup | Connect | PreTransfer | StartTransfer | Total => format!("{:<7}", elapsed),
        }
    }
}

/// Output template
pub struct Template {
    progresses: HashMap<String, String>,
    scheme: String,
}

impl Template {
    /// Returns Template
    pub fn new(scheme: &str) -> Template {
        Template {
            progresses: HashMap::new(),
            scheme: scheme.to_owned(),
        }
    }

    /// Returns strings for output
    pub fn format(&self) -> String {
        match self.scheme {
            ref a if a == "https" => self.https_format(),
            _ => self.http_format(),
        }
    }

    /// Add a progress
    pub fn progress(&mut self, progress: Progress) {
        let (index, time) = progress.output();
        self.progresses.insert(index, time);
    }

    /// HTTP format
    fn http_format(&self) -> String {
        format!("  \
DNS Lookup   TCP Connection   Server Processing   Content Transfer
[   {a0000}  |     {a0001}    |      {a0003}      |      {a0004}     ]
             |                |                   |                  |
    namelookup:{b0000}        |                   |                  |
                        connect:{b0001}           |                  |
                                      starttransfer:{b0003}          |
                                                                 total:{b0004}",
                a0000 = self.progresses.get("a0000").unwrap_or(&"".to_owned()),
                a0001 = self.progresses.get("a0001").unwrap_or(&"".to_owned()),
                a0003 = self.progresses.get("a0003").unwrap_or(&"".to_owned()),
                a0004 = self.progresses.get("a0004").unwrap_or(&"".to_owned()),
                b0000 = self.progresses.get("b0000").unwrap_or(&"".to_owned()),
                b0001 = self.progresses.get("b0001").unwrap_or(&"".to_owned()),
                b0003 = self.progresses.get("b0003").unwrap_or(&"".to_owned()),
                b0004 = self.progresses.get("b0004").unwrap_or(&"".to_owned()))
    }

    /// HTTPS format
    fn https_format(&self) -> String {
        format!("  \
DNS Lookup   TCP Connection   SSL Handshake   Server Processing   Content Transfer
[   {a0000}  |     {a0001}    |    {a0002}    |      {a0003}      |      {a0004}     ]
             |                |               |                   |                  |
    namelookup:{b0000}        |               |                   |                  |
                        connect:{b0001}       |                   |                  |
                                    pretransfer:{b0002}           |                  |
                                                      starttransfer:{b0003}          |
                                                                                 total:{b0004}",
                a0000 = self.progresses.get("a0000").unwrap_or(&"".to_owned()),
                a0001 = self.progresses.get("a0001").unwrap_or(&"".to_owned()),
                a0002 = self.progresses.get("a0002").unwrap_or(&"".to_owned()),
                a0003 = self.progresses.get("a0003").unwrap_or(&"".to_owned()),
                a0004 = self.progresses.get("a0004").unwrap_or(&"".to_owned()),
                b0000 = self.progresses.get("b0000").unwrap_or(&"".to_owned()),
                b0001 = self.progresses.get("b0001").unwrap_or(&"".to_owned()),
                b0002 = self.progresses.get("b0002").unwrap_or(&"".to_owned()),
                b0003 = self.progresses.get("b0003").unwrap_or(&"".to_owned()),
                b0004 = self.progresses.get("b0004").unwrap_or(&"".to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Process::*;
    use format::Format::*;
    use progress::Progress;

    #[test]
    fn process_index() {
        assert_eq!(DnsLookup.index(), "a0000");
        assert_eq!(TcpConnection.index(), "a0001");
        assert_eq!(SslHandshake.index(), "a0002");
        assert_eq!(ServerProcessing.index(), "a0003");
        assert_eq!(ContentTransfer.index(), "a0004");
        assert_eq!(NameLookup.index(), "b0000");
        assert_eq!(Connect.index(), "b0001");
        assert_eq!(PreTransfer.index(), "b0002");
        assert_eq!(StartTransfer.index(), "b0003");
        assert_eq!(Total.index(), "b0004");
    }

    #[test]
    fn http_format() {
        let mut t = Template::new("http");
        t.progress(Progress::new(DnsLookup, 100, Blue));
        t.progress(Progress::new(TcpConnection, 200, Blue));
        t.progress(Progress::new(ServerProcessing, 300, Blue));
        t.progress(Progress::new(ContentTransfer, 400, Blue));
        t.progress(Progress::new(NameLookup, 500, Blue));
        t.progress(Progress::new(Connect, 600, Blue));
        t.progress(Progress::new(StartTransfer, 700, Blue));
        t.progress(Progress::new(Total, 800, Blue));

        assert_eq!(t.format(),
                   "  DNS Lookup   TCP Connection   Server Processing   Content Transfer
[   \
                    \x1b[34m 100ms \x1b[0m  |     \x1b[34m 200ms \x1b[0m    |      \
                    \x1b[34m 300ms \x1b[0m      |      \x1b[34m 400ms \x1b[0m     ]
             |                \
                    |                   |                  |
    namelookup:\x1b[34m500ms  \x1b[0m        |                   \
                    |                  |
                        connect:\x1b[34m600ms  \x1b[0m           |                  \
                    |
                                      starttransfer:\x1b[34m700ms  \x1b[0m          |
                                                                 \
                    total:\x1b[34m800ms  \x1b[0m");
    }

    #[test]
    fn https_format() {
        let mut t = Template::new("https");
        t.progress(Progress::new(DnsLookup, 100, Red));
        t.progress(Progress::new(TcpConnection, 200, Red));
        t.progress(Progress::new(SslHandshake, 300, Red));
        t.progress(Progress::new(ServerProcessing, 400, Red));
        t.progress(Progress::new(ContentTransfer, 500, Red));
        t.progress(Progress::new(NameLookup, 600, Red));
        t.progress(Progress::new(Connect, 700, Red));
        t.progress(Progress::new(PreTransfer, 800, Red));
        t.progress(Progress::new(StartTransfer, 900, Red));
        t.progress(Progress::new(Total, 1000, Red));

        assert_eq!(t.format(),
                   "  DNS Lookup   TCP Connection   SSL Handshake   Server Processing   Content \
                    Transfer
[   \x1b[31m 100ms \x1b[0m  |     \x1b[31m 200ms \x1b[0m    |    \
                    \x1b[31m 300ms \x1b[0m    |      \x1b[31m 400ms \x1b[0m      |      \
                    \x1b[31m 500ms \x1b[0m     ]
             |                |               |                   \
                    |                  |
    namelookup:\x1b[31m600ms  \x1b[0m        |               |                   \
                    |                  |
                        connect:\x1b[31m700ms  \x1b[0m       |                   \
                    |                  |
                                    pretransfer:\x1b[31m800ms  \x1b[0m           \
                    |                  |
                                                      \
                    starttransfer:\x1b[31m900ms  \x1b[0m          |
                                                                                 \
                    total:\x1b[31m1000ms \x1b[0m");
    }
}
