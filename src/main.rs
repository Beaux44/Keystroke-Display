use inputbot::{KeybdKey, KeybdKey::*};
use inputbot;
use message_io::node::{self, NodeEvent};
use message_io::network::{NetEvent, Transport, Endpoint};
use serde::{Serialize, Deserialize};
use std::io::Write;
use std::{thread, env, process};


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event")]
enum Signal {
    KeyPress{ key: String, ctrl: bool, shift: bool },
}

fn main() {
    const TRANSPORT: Transport = Transport::Ws;
    let args: Vec<String> = env::args().collect();
    let addr: String;
    let mut port = 17508;

    if args.len() > 1 {
        match args[1].parse::<u16>() {
            Ok(p) => port = p,
            Err(_) => {
                println!("Invalid port number");
                process::exit(1);
            },
        }
    }

    addr = format!("127.0.0.1:{}", port);

    match std::fs::File::create("overlay/port.js") {
        Ok(mut f) => {
            f.write_all(format!("let address = 'ws://{}/';", addr).as_bytes()).unwrap();
        },
        Err(_) => {
            println!("Could not create port.js file");
            process::exit(1);
        },
    };

    let (handler, listener) = node::split();

    match handler.network().listen(TRANSPORT, &addr) {
        Ok((_id, real_addr)) => println!("Server running at {} by {}", real_addr, TRANSPORT),
        Err(_) => return println!("Can not listen at {} by {}", addr, TRANSPORT),
    };


    let handler2 = handler.clone();

    KeybdKey::bind_all(move |event| {
        match event {
            LShiftKey | RShiftKey | LControlKey => {}
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
            NetEvent::Disconnected(endpoint) => {
                clients.retain(|&x| x != endpoint);
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

