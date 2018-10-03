extern crate mio;
extern crate http_muncher;

use http_muncher::{Parser, ParserHandler};
use mio::*;
use mio::tcp::*;
use std::net::SocketAddr;
use std::collections::HashMap;

const SERVER_TOKEN: Token = Token(0);

struct HttpParser;

impl ParserHandler for HttpParser {

}

struct WebSocketClient {
    socket: TcpStream,
    http_parser: Parser<HttpParser>
}

impl WebSocketClient {
    fn read(&mut self){
        loop {
            let mut buf = [0; 2048];
            match self.socket.try_read(&mut buf){
                Err(e) =>
                    println!("Error while attempting to read socket! {:?}", e);
            },
            Ok(None) => break,
            Ok(Some(len)) => {
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
            http_parser: Parser::Request(HttpParser);
        }
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

fn main() {

    let mut address = "0.0.0.0:0000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    let mut event_loop = EventLoop::new().unwrap();
    let mut handler = WebSocketServer;
    event_loop.run(&mut handler).unwrap();

    event_loop.register(&server_socket, Token(0), EventSet::readable(), PollOpt::edge()).unwrap();
}
