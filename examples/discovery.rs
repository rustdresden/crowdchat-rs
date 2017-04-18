extern crate crowdchat;

use std::io::Write;
use std::net::TcpListener;

use crowdchat::clients;


fn accept_connections(listener: TcpListener) {
    let mut clients = vec!();
    let mut socks = vec!();
    loop {
        let (new_socket, addr) = listener.accept().unwrap();
        let stream_name = &addr.ip().to_string();
        let response = clients::handle_client(&mut clients, stream_name);
        let response_string = response.to_string();
        socks.push(new_socket);
        for mut socket in socks.iter() {
            socket.write(response_string.as_bytes()).unwrap();
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:17424").unwrap();
    accept_connections(listener);
}
