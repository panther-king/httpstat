/// Times of HTTP/HTTPS connection.
use format::Format;
use template::Process;

/// Progress of each process
pub struct Progress {
    formatter: Format,
    process: Process,
    ms: usize,
}

impl Progress {
    /// Returns Progress
    pub fn new(process: Process, ms: usize, formatter: Format) -> Progress {
        Progress {
            formatter: formatter,
            process: process,
            ms: ms,
        }
    }

    /// Returns a elapsed time for output
    pub fn output(&self) -> (String, String) {
        let time = format!("{}ms", self.ms);

        (self.process.index(), self.formatter.tty(&time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use format::Format::*;
    use template::Process::*;

    #[test]
    fn output_progress() {
        let p = Progress::new(DnsLookup, 100, Blue);
        let (index, format) = p.output();

        assert_eq!(index, "a0000");
        assert_eq!(format, "\x1b[34m100ms\x1b[0m");
    }
}
