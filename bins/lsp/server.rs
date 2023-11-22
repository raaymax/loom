use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, self, Error};
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize)]
struct Rpc {
    jsonrpc: String,
    method: String,
    id: usize,
    params: serde_json::Value,
}

fn parse_content_length(buffer: &str) -> Result<usize, Error> {
    let reg = Regex::new(r"Content-Length: (\d+)").unwrap();
    let Some(res) = reg.captures(buffer) else {
        return Err(Error::new(io::ErrorKind::Other, "Cannot parse Content-Length"));
    };
    let Some(mat) = res.get(1) else {
        return Err(Error::new(io::ErrorKind::Other, "Cannot parse Content-Length2 "));
    };
    mat.as_str().parse::<usize>().map_err(|e| Error::new(io::ErrorKind::Other, e))
}

pub struct Response<'a> {
    id: usize,
    transport: &'a mut Transport,
}

impl<'a> Response<'a> {
    fn new(transport: &'a mut Transport, id: usize) -> Self {
        Response { id, transport }
    }
    pub fn send(&mut self, res: serde_json::Value) -> Result<(), Error> {
        let packet = serde_json::json!({
            "jsonrpc": "2.0",
            "id": self.id,
            "result": res,
        });
        self.transport.send(packet.to_string())
    }
}

pub struct Request<'a>{
    pub id: usize,
    pub method: String,
    pub params: serde_json::Value,
    _log: &'a mut File,
}

impl<'a> Request<'a> {
    fn new(id: usize, method: String, params: serde_json::Value, log: &'a mut File) -> Self {
        Request { id, method, params, _log:log }
    }
    pub fn log(&mut self, message: &str) {
        writeln!(self._log, "{}", message);
    }
}

pub struct Transport {
    reader: Box<dyn BufRead>,
    writer: Box<dyn Write>,
}

impl Transport {
    pub fn new(reader: Box<dyn BufRead>, writer: Box<dyn Write>) -> Self {
        Transport { reader, writer }
    }
    pub fn send(&mut self, message: String) -> Result<(), Error> {
        write!(self.writer, "Content-Length: {}\r\n\r\n{}", message.len(), message)
    }
    pub fn receive(&mut self) -> Result<Rpc, Error> {
        let mut buffer = String::new();
        let mut trash =vec![0; 10];

        self.reader.read_line(&mut buffer)?;
        let content_length = parse_content_length(&buffer)?;
        self.reader.read_until(b'\n', &mut trash)?;
        let mut packet = vec![0; content_length];
        self.reader.read_exact(&mut packet)?;
        serde_json::from_slice(&packet).map_err(|e| Error::new(io::ErrorKind::Other, e))
    }
}
type Handler = fn(&mut Request, &mut Response) -> Result<(), Error>;

pub struct Server {
    transport: Transport,
    handlers: HashMap<String, Handler>,
    log: File,
}

impl Server {
    pub fn new(transport: Transport) -> Self {
        Server {
            transport, 
            handlers: HashMap::new(),
            log: File::create("foo.log").unwrap(),
        }
    }
    pub fn add_method(mut self, method: &str, handler: Handler ) -> Self {
        self.handlers.insert(method.to_string(), handler);
        self
    }
    pub fn add_fallback(mut self, handler: Handler) -> Self {
        self.handlers.insert("fallback".to_string(), handler);
        self
    }


    pub fn listen(&mut self) {
        while let Ok(()) = self.tick() {}
    }
    pub fn tick(&mut self) -> Result<(), Error> {
        let packet = self.transport.receive().unwrap();
        let handler = self.handlers.get(&packet.method).unwrap_or_else(|| {
            self.handlers.get("fallback").unwrap()
        });
        writeln!(self.log, "Received id: {} method: {}\n", packet.id, packet.method);
        writeln!(self.log, "Received params: {}\n", packet.params);
        {
            let mut req = Request::new(packet.id, packet.method, packet.params, &mut self.log);
            let mut res = Response::new(&mut self.transport, packet.id);
            handler(&mut req, &mut res)
        }
    }
}
