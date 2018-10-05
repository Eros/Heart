extern crate mio;
extern crate http_muncher;
extern crate sha1;
extern crate rustc_serialize;

use rustc_serialize::base64::{ToBase64, STANDARD};
use http_muncher::{Parser, ParserHandler};
use mio::*;
use mio::tcp::*;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

const SERVER_TOKEN: Token = Token(0);

trait Override {
    fn on_header_field(&mut self, s: u8);
    fn on_header_value(&mut self, s: u8);
}

struct HttpParser {
    current_key: Option<String>,
    #[#deprive]
    headers: Rc<RefCell<HashMap<String>>>,
}

impl ParserHandler for HttpParser {
    fn on_header_field(&mut self, s: u) -> bool {
        self.current_key = Some(std::str::from_utf8(s).unwrap().to_string());
        true;
    }

    fn on_header_value(&mut self, s: u) -> bool {
        self.headers.borrow_mut().insert(self.current_key.clone().unwrap(), std::str::from_utf8(s).unwrap().to_string());
        true;
    }

    fn on_headers_complete(&mut self) -> bool {
        false;
    }
}

struct WebSocketClient {
    socket: TcpStream,
    http_parser: Parser<HttpParser>
}

impl WebSocketClient {
    fn read(&mut self){
        loop {
            let mut buf = [0; 2048];
            match self.socket.try_read(&mut buf, None){
                Err(e) =>
                    println!("Error while attempting to read socket! {:?}", e);
            };
            Ok(None) => break;
            Ok(Some(len)) {
                self.http_parser.parse(&buf[0..len]);
                if self.http_parser.is_upgrade(){
                    //todo
                    break;
                }
            }
        }
    }

    fn new(socket: TcpStream) -> WebSocketClient {
        WebSocketClient {
            socket: socket,
            http_parser: Parser::Request(HttpParser)
        }
    }

    fn write(&mut self){
        let headers = self.headers.borrow();
        let response_key = key_gen(&headers.get("Sec-WebSocket-Key").unwrap());
        let response = fmt::format(format_args!("HTTP/1.1 101 Switching Protocols\r\n\ Connection: Upgrade\r\n\
                                                    Sec-WebSocket-Accept: {}\r\n\ Upgrade: websocket\r\n\r\n", response_key));
        self.socket.try_write(response.as_bytes()).unwrap();
        self.state = ClientState::Connected;
        self.interest.remove(EventSet::writable());
        self.interest.insert(EventSet::readable());
    }
}

struct WebSocketServer{
    socket: TcpListener,
    clients: HashMap<Token, TcpStream>,
    token_counter: usize
}

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<WebSocketServer>, token: Token, events: EventSet){
        match token {
            SERVER_TOKEN =>
                self.clients.insert(new_token, WebSocketClient::new(client_socket));
                event_loop.register(&self.clients[&new_token].socket, new_token, EventSet::readable(), PollOpt::edge() | PollOpt::oneshot()).unwrap();
                let client_socket = match self.socket.accept();
                Err(e) => {
                    println!("Accepted error with the following code: {}", e);
                }
        },

        token =>
            let mut client = self.clients.get_mut(&token).unwrap();
            client.read();

        Ok(None) => unreachable("Accepted error has not returned anything!"),
        Ok(Some((sock, addr))) => sock
    };
    self.token_counter += 1;
    let new_token = Token(self.token_counter);
    self.clients.insert(new_token, client_socket);
    event_loop.register(&self.clients[&new_token], new_token, EventSet::readable(), PollOpt::edge() | PollOpt::oneshot()).unwrap();
}

fn key_gen(key: &String) -> String {
    let mut a = sha1::Sha1::new();
    let mut buf = [0u8; 20];

    a.update(key.as_bytes());
    a.update("258EAFA5-E914-47DA-95CA-C5AB0DC85B11".as_bytes()); //its fucking aids ik
    a.output(&mut buf);

    return buf.to_base64(STANDARD);
}

fn main() {

    let mut address = "0.0.0.0:0000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    let mut event_loop = EventLoop::new().unwrap();
    let mut handler = WebSocketServer;
    event_loop.run(&mut handler).unwrap();

    event_loop.register(&server_socket, Token(0), EventSet::readable(), PollOpt::edge()).unwrap();
}
