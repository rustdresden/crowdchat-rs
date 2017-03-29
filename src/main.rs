extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::io;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

static recv_address: &'static str = "172.16.1.18:54321";
static send_address: &'static str = "172.16.1.18:54322";

static jk_address: &'static str = "172.16.1.1:54321";
static hoodie_address: &'static str = "172.16.1.187:54321";
static henrik_address: &'static str = "172.16.1.18:54321";

static broadcast_address: &'static str = "255.255.255.255:54321";

struct message {
    name: String,
    content: String,
}

fn main() {
    let mut recv_socket = UdpSocket::bind(recv_address).unwrap();
    let mut send_socket = UdpSocket::bind(send_address).unwrap();

    let send_thread = thread::spawn(move || {
        let mut input = String::new();

        send_socket.set_broadcast(true).unwrap();

        while true {
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    println!("Sending: {}", input);
                    send_socket.send_to(input.as_bytes(), hoodie_address);
                    send_socket.send_to(input.as_bytes(), jk_address);
                    send_socket.send_to(input.as_bytes(), henrik_address);
                    send_socket.send_to(input.as_bytes(), broadcast_address);
                }
                Err(error) => println!("error: {}", error),
            }
        }
    });

    let receive = thread::spawn(move || {
        // recv_socket.set_broadcast(true).unwrap();

        while true {
            let mut buf = [0; 4096];
            let (amt, src) = recv_socket.recv_from(&mut buf).unwrap();


            println!("{}: {}", src, String::from_utf8_lossy(&buf));
        }
    });


    send_thread.join().unwrap();
}
