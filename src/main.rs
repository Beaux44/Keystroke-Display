use inputbot::{KeybdKey, KeybdKey::*};
use inputbot;
use message_io::node::{self, NodeEvent};
use message_io::network::{NetEvent, Transport, Endpoint};
use serde::{Serialize, Deserialize};
use std::thread;


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event")]
enum Signal {
    KeyPress{ key: String, ctrl: bool, shift: bool },
}

fn main() {
    const TRANSPORT: Transport = Transport::Ws;
    const ADDRESS: &str = "127.0.0.1:80";
    let (handler, listener) = node::split();

    match handler.network().listen(TRANSPORT, ADDRESS) {
        Ok((_id, real_addr)) => println!("Server running at {} by {}", real_addr, TRANSPORT),
        Err(_) => return println!("Can not listen at {} by {}", ADDRESS, TRANSPORT),
    };


    let handler2 = handler.clone();

    KeybdKey::bind_all(move |event| {
        match event {
            LShiftKey | LControlKey => {}
            _ => handler.signals()
                        .send(Signal::KeyPress{
                                key: format!("{:?}", event),
                                ctrl: LControlKey.is_pressed(),
                                shift: LShiftKey.is_pressed() || RShiftKey.is_pressed()
                        })
        }
    });

    thread::spawn(inputbot::handle_input_events);

    let mut clients: Vec<Endpoint> = Vec::new();
    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(_endpoint, _ok) => {}
            NetEvent::Accepted(endpoint, _) => {
                println!("Accepted connection from {}", endpoint);
                clients.push(endpoint);
            }
            NetEvent::Message(_endpoint, data) => {
                let data: Signal = serde_json::from_slice(&data).unwrap();
                println!("Received: {:?}", data);
            },
            NetEvent::Disconnected(_endpoint) => {
                println!("Client disconnected from server");
            },
        }

        NodeEvent::Signal(signal) => match signal {
            Signal::KeyPress { .. } => {
                    clients.iter().for_each(|client| {
                        handler2.network().send(*client, &serde_json::to_vec(&signal).unwrap()[..]);
                    });
            }
        }
    });
}

