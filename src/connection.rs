//! HTTP/HTTPS request
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::process::Command;

pub struct Connection {
    command: Command,
}

impl Connection {
    /// Returns Connection with a curl command
    pub fn with_curl(args: Vec<String>) -> Connection {
        let excludes =
            vec!["-w", "--write-out", "-D", "--dump-header", "-o", "--output", "-s", "--silent"];
        let mut command = Command::new("curl");

        for arg in args.iter() {
            match excludes.iter().find(|&e| e == &arg) {
                Some(_) => panic!(""),
                None => {
                    command.arg(arg);
                }
            }
        }

        Connection { command: command }
    }

    /// Execute curl
    pub fn execute(&mut self, url: &str) -> Response {
        self.add_write_out();
        self.add_dump_header();
        self.add_output();
        self.command.arg("-s");
        self.command.arg("-S");
        self.command.arg(url);

        let response = match self.command.output() {
            Ok(o) => String::from_utf8(o.stdout).unwrap_or("".to_owned()),
            Err(e) => "".to_owned(),
        };
        let times = response.split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&s| s.to_owned())
            .collect::<Vec<String>>();

        Response::new(times)
    }

    /// Add write out option
    fn add_dump_header(&mut self) {
        let mut dir = env::temp_dir();

        dir.push("httpstat_dump_header.txt");
        File::create(&dir);

        self.command.arg("-D");
        self.command.arg(dir.to_str().unwrap());
    }

    /// Add output option
    fn add_output(&mut self) {
        let mut dir = env::temp_dir();

        dir.push("httpstat_output.txt");
        File::create(&dir);

        self.command.arg("-o");
        self.command.arg(dir.to_str().unwrap());
    }

    /// Add write out option
    fn add_write_out(&mut self) {
        let write_out = ["time_namelookup:%{time_namelookup}",
                         "time_connect:%{time_connect}",
                         "time_appconnect:%{time_appconnect}",
                         "time_pretransfer:%{time_pretransfer}",
                         "time_starttransfer:%{time_starttransfer}",
                         "time_total:%{time_total}",
                         "speed_download:%{speed_download}",
                         "speed_upload:%{speed_upload}"];

        self.command.arg("-w");
        self.command.arg(write_out.join("\n"));
    }
}

pub struct Response {
    processes: HashMap<String, f32>,
}

impl Response {
    /// Returns Response
    pub fn new(times: Vec<String>) -> Response {
        let mut parsed = HashMap::new();

        for t in times.iter() {
            let kv = t.split(':').collect::<Vec<_>>();
            parsed.insert(kv[0].trim().to_owned(),
                          kv[1].trim().parse::<f32>().unwrap_or(0f32));
        }

        Response { processes: parsed }
    }

    /// Returns millisecond of range connection
    pub fn range_connection(&self) -> usize {
        self.time_connect() - self.time_namelookup()
    }

    /// Returns millisecond of range DNS
    pub fn range_dns(&self) -> usize {
        self.time_namelookup()
    }

    /// Returns millisecond of range server
    pub fn range_server(&self) -> usize {
        self.time_starttransfer() - self.time_pretransfer()
    }

    /// Returns millisecond of range SSL
    pub fn range_ssl(&self) -> usize {
        self.time_pretransfer() - self.time_connect()
    }

    /// Returns millisecond of range transfer
    pub fn range_transfer(&self) -> usize {
        self.time_total() - self.time_starttransfer()
    }

    /// Returns millisecond of %{time_connect}
    pub fn time_connect(&self) -> usize {
        (*self.processes.get("time_connect").unwrap() * 1_000f32) as usize
    }

    /// Returns millisecond of %{time_namelookup}
    pub fn time_namelookup(&self) -> usize {
        (*self.processes.get("time_namelookup").unwrap() * 1_000f32) as usize
    }

    /// Returns millisecond of %{time_pretransfer}
    pub fn time_pretransfer(&self) -> usize {
        (*self.processes.get("time_pretransfer").unwrap() * 1_000f32) as usize
    }

    /// Returns millisecond of %{time_starttransfer}
    pub fn time_starttransfer(&self) -> usize {
        (*self.processes.get("time_starttransfer").unwrap() * 1_000f32) as usize
    }

    /// Returns millisecond of %{time_total}
    pub fn time_total(&self) -> usize {
        (*self.processes.get("time_total").unwrap() * 1_000f32) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response() -> Response {
        Response::new(vec![
            "time_namelookup:2.512".to_owned(),
            "time_connect:2.598".to_owned(),
            "time_appconnect:0".to_owned(),
            "time_pretransfer:2.599".to_owned(),
            "time_starttransfer:2.659".to_owned(),
            "time_total:2.782".to_owned(),])
    }

    #[test]
    fn range_connection_of_response() {
        assert_eq!(response().range_connection(), 86);
    }

    #[test]
    fn range_dns_of_response() {
        assert_eq!(response().range_dns(), 2512);
    }

    #[test]
    fn range_server_of_response() {
        assert_eq!(response().range_server(), 60);
    }

    #[test]
    fn range_ssl_of_reponse() {
        assert_eq!(response().range_ssl(), 1);
    }

    #[test]
    fn range_transfer_of_response() {
        assert_eq!(response().range_transfer(), 123);
    }

    #[test]
    fn time_connect_of_response() {
        assert_eq!(response().time_connect(), 2598);
    }

    #[test]
    fn time_namelookup_of_response() {
        assert_eq!(response().time_namelookup(), 2512);
    }

    #[test]
    fn time_pretransfer_of_response() {
        assert_eq!(response().time_pretransfer(), 2599);
    }

    #[test]
    fn time_starttransfer_of_response() {
        assert_eq!(response().time_starttransfer(), 2659);
    }

    #[test]
    fn time_total_of_response() {
        assert_eq!(response().time_total(), 2782);
    }
}
