extern crate mio;
use mio::*;
use mio::tcp::*;
use std::net::SocketAddr;
use std::collections::HashMap;

static const SERVER_TOKEN: Token = Token(0);

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
                let client_socket = match self.socket.accept();
                Err(e) => {
                    println!("Accepted error with the following code: {}", e);
                }
        },
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
