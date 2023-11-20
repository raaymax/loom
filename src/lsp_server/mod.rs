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

//parse context-length using regex
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


fn get_capabilities(id: usize) -> serde_json::Value {
    let response = serde_json::json!({
        "capabilities": {
            "textDocumentSync": 0,
            "semanticTokensProvider": {
                "id": 0,
                "method": "getTokens",
            }
        },
        "serverInfo": {
            "name": "langvm",
            "version": "0.0.1",
        }
    });
    response 
}

struct Response<'a> {
    id: usize,
    transport: &'a mut Transport,
}

impl<'a> Response<'a> {
    fn new(transport: &'a mut Transport, id: usize) -> Self {
        Response { id, transport }
    }
    fn send(&mut self, res: serde_json::Value) -> Result<(), Error> {
        let packet = serde_json::json!({
            "jsonrpc": "2.0",
            "id": self.id,
            "result": res,
        });
        self.transport.send(packet.to_string())
    }
}

struct Request<'a>{
    id: usize,
    method: String,
    params: serde_json::Value,
    log: &'a mut File,
}

impl<'a> Request<'a> {
    fn new(id: usize, method: String, params: serde_json::Value, log: &'a mut File) -> Self {
        Request { id, method, params, log }
    }
    fn log(&mut self, message: &str) {
        writeln!(self.log, "{}", message);
    }
}

struct Transport {
    reader: Box<dyn BufRead>,
    writer: Box<dyn Write>,
}

impl Transport {
    fn new(reader: Box<dyn BufRead>, writer: Box<dyn Write>) -> Self {
        Transport { reader, writer }
    }
    fn send(&mut self, message: String) -> Result<(), Error> {
        write!(self.writer, "Content-Length: {}\r\n\r\n{}", message.len(), message)
    }
    fn receive(&mut self) -> Result<Rpc, Error> {
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

struct Server {
    transport: Transport,
    handlers: HashMap<String, Handler>,
    log: File,
}

impl Server {
    fn new(transport: Transport) -> Self {
        Server {
            transport, 
            handlers: HashMap::new(),
            log: File::options().append(true).create(true).open("foo.log").unwrap(),
        }
    }
    fn add_method(mut self, method: &str, handler: Handler ) -> Self {
        self.handlers.insert(method.to_string(), handler);
        self
    }
    fn add_fallback(mut self, handler: Handler) -> Self {
        self.handlers.insert("fallback".to_string(), handler);
        self
    }


    fn listen(&mut self) {
        while let Ok(()) = self.tick() {}
    }
    fn tick(&mut self) -> Result<(), Error> {
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

fn handle_initialize(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    let id = req.id;
    req.log(format!("initialize id: {}", id).as_str());
    let caps = get_capabilities(id);
    res.send(caps)
}

fn handle_fallback(req: &mut Request, res: &mut Response) -> Result<(), Error> {
    let id = req.id;
    req.log(format!("initialize id: {}", id).as_str());
    req.log(format!("{} id: {}", req.method, id).as_str());
    req.log(format!("{:?}", req.params).as_str());
    let response = serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": "fallback",
    });
    res.send(response)
}

pub fn run() {
    Server::new(Transport::new(Box::new(io::stdin().lock()), Box::new(io::stdout().lock())))
        .add_method("initialize", handle_initialize)
        .add_fallback(handle_fallback)
        .listen();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_test_message() {
        let message = r#"Content-Length: 65

{"jsonrpc": "2.0", "method": "initialize", "id": 1, "params": {}}"#;
        let reader = Box::new(message.as_bytes()) as Box<dyn BufRead>;
        let writer = Box::new(Vec::new());

        Server::new(Transport::new(reader, writer))
            .add_method("initialize", |rpc, res| {
                let id = rpc.id;
                assert_eq!(id, 1);
                assert_eq!(rpc.method, "initialize");
                assert_eq!(rpc.params, serde_json::json!({}));
                let caps = get_capabilities(id);
                res.send(caps)
            })
            .tick();
    }

    #[test]
    fn test_use_fallback() {
        let message = r#"Content-Length: 65

{"jsonrpc": "2.0", "method": "initialize", "id": 1, "params": {}}"#;
        let reader = Box::new(message.as_bytes()) as Box<dyn BufRead>;
        let writer = Box::<Vec<u8>>::default();

        Server::new(Transport::new(reader, writer))
            .add_fallback(|rpc, res| {
                let id = rpc.id;
                assert_eq!(id, 1);
                assert_eq!(rpc.method, "initialize");
                assert_eq!(rpc.params, serde_json::json!({}));
                let caps = get_capabilities(id);
                res.send(caps)
            })
            .tick();
    }
    
}
