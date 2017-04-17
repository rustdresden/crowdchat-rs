extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::io;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

static recv_address: &'static str = "0.0.0.0:54321";
static send_address: &'static str = "localhost:54322";
static server_address: &'static str = "localhost:17424";

static RECEIVER_ADDRESSES: &'static [&'static str] = &[
    "192.168.43.72:54321",
    "192.168.43.154:54321",
    "192.168.43.239:54321",
    "192.168.43.90:54321",
    "192.168.43.255:54321"
];


static broadcast_address: &'static str = "255.255.255.255:54321";

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    name: String,
    content: ContentType,
}

#[derive(Debug, Serialize, Deserialize)]
enum ContentType{
    Message(String),
    Arrived,
    Annouce,
}

fn broadcast(send_socket: &UdpSocket, msg: &Message) {
    let wire_format = serde_json::to_string(&msg).unwrap();

    for addr in RECEIVER_ADDRESSES.iter() {
        send_socket.send_to(wire_format.as_bytes(),addr);
    }
}

fn make_message(content: ContentType) -> Message {
    Message {
        name: String::from("Hendrik"),
        content: content
    }
}

fn main() {
    let mut recv_socket = UdpSocket::bind(recv_address).unwrap();
    let mut send_socket = UdpSocket::bind(send_address).unwrap();

    let send_thread = thread::spawn(move || {

        send_socket.set_broadcast(true).unwrap();

        while true {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let msg = Message{
                        name: String::from("hoodie"),
                        content: ContentType::Message(input.trim().into())
                    };
                    broadcast(&send_socket, &msg);
                }
                Err(error) => println!("error: {}", error),
            }
        }
    });

    let receive = thread::spawn(move || {
        // recv_socket.set_broadcast(true).unwrap();

        while true {
            let mut buf = [0; 4096];
            let (len, src) = recv_socket.recv_from(&mut buf).unwrap();
            let received = String::from_utf8_lossy(&buf[0..len]);
            match serde_json::from_str::<Message>(&received) {
                Ok(msg) => {
                    match msg.content {
                        ContentType::Message(text) => println!("{}, {:?}", msg.name, text),
                        ContentType::Arrived => {
                            println!("-- {} entered the room", msg.name);
                        },
                        _ => println!("unhandled message")
                    }
                },
                Err(e) => println!("bad message format {:?}", e)
            }


        }
    });


    receive.join().unwrap();
    send_thread.join().unwrap();
}
