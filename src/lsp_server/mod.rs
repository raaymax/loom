mod server;

use std::io::{self, Error};
use self::server::{Request, Response, Transport, Server};

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
    use std::io::BufRead;

    use super::*;

    #[test]
    fn test_parse_test_message() {
        let message = r#"Content-Length: 65

{"jsonrpc": "2.0", "method": "initialize", "id": 1, "params": {}}"#;
        let reader = Box::new(message.as_bytes()) as Box<dyn BufRead>;
        let writer = Box::new(Vec::new());

        let _ = Server::new(Transport::new(reader, writer))
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

        let _ = Server::new(Transport::new(reader, writer))
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
