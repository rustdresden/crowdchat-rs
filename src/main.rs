extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::io;
use std::io::Read;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

static recv_address:&'static str = "0.0.0.0";
static recv_port:&'static str = "54321";
static server_address: &'static str = "harkness:17424";

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

fn broadcast(receiver_addresses: &Vec<String>, send_socket: &UdpSocket, msg: &Message) {
    let wire_format = serde_json::to_string(&msg).unwrap();

    for addr in receiver_addresses.iter() {
        send_socket.send_to(wire_format.as_bytes(),addr).unwrap();
    }
}

fn main() {
    let mut receiver_addresses = Arc::new(Mutex::new(vec!()));
    let mut socket = Arc::new(UdpSocket::bind(format!("{}:{}", recv_address, recv_port)).unwrap());

    let mut discover_receiver_addresses = receiver_addresses.clone();
    let discover_thread = thread::spawn(move || {
        let mut discover_socket = TcpStream::connect(server_address).unwrap();
        let mut first_read = true;
        loop {
            let mut buffer = [0; 512];
            let count = discover_socket.read(&mut buffer[..]).unwrap();
            if count == 0 {
                println!("Server returned empty response, aborting discovery!");
                break;
            }

            let mut receiver_addresses = discover_receiver_addresses.lock().unwrap();
            receiver_addresses.clear();
            let json_response : serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&buffer[0..count])).unwrap();
            for client in json_response["clients"].as_array().unwrap() {
                let mut new_address = format!("{}:{}", client["address"].as_str().unwrap(), recv_port);
                receiver_addresses.push(new_address);
            }
        }
    });

    let send_receiver_addresses = receiver_addresses.clone();
    let mut send_socket = socket.clone();
    let send_thread = thread::spawn(move || {

        while true {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let msg = Message{
                        name: String::from("hoodie"),
                        content: ContentType::Message(input.trim().into())
                    };
                    let receiver_addresses = send_receiver_addresses.lock().unwrap();
                    broadcast(&receiver_addresses, &send_socket, &msg);
                }
                Err(error) => println!("error: {}", error),
            }
        }
    });

    let mut recv_socket = socket.clone();
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
    discover_thread.join().unwrap();
}
