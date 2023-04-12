use std::time::Duration;
use inputbot::{KeybdKey, KeybdKey::*};
use inputbot;
use message_io::node::{self, NodeEvent, NodeHandler};
use message_io::network::{NetEvent, Transport, Endpoint};
use serde::{Serialize, Deserialize};
use std::{thread, process};


macro_rules! unwrap_or_return {
    ( $e: expr ) => {
        match $e {
            Ok(res) => res,
            Err(_) => return
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event")]
enum Signal {
    KeyPress{ key: String, ctrl: bool, shift: bool },
}

fn connect_server(handler: &NodeHandler<Signal>, addr: &str) -> Result<Endpoint, ()> {
    for _ in 0..5 {
        match handler.network().connect_sync(Transport::Ws, addr) {
            Ok((server, _)) => {
                println!("Successfully connected to server");
                return Ok(server);
            }
            Err(e) => println!("Failed to connect to server, {:?}", e)
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    println!("Failed to connect to server, exiting");
    Err(())
}

fn main() {
    const ADDRESS: &str = "ws://127.0.0.1:80/keybd";
    let (handler, listener) = node::split();
    let server = unwrap_or_return!(connect_server(&handler, ADDRESS));

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

    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(_endpoint, _ok) => {}
            NetEvent::Accepted(_, _) => unreachable!(),
            NetEvent::Message(_endpoint, data) => {
                let data: Signal = bson::from_slice(data).unwrap();
                println!("Received: {:?}", data);
            },
            NetEvent::Disconnected(_endpoint) => {
                println!("Disconnected from server, exiting");
                process::exit(0);
            },
        }

        NodeEvent::Signal(signal) => match signal {
            Signal::KeyPress { .. } => {
                    handler2.network().send(server, &bson::to_vec(&signal).unwrap()[..]);
            }
        }
    });
}

