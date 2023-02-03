use std::sync::{Arc, Mutex};
use inputbot::{KeybdKey, MouseButton};
use inputbot;
use chrono::prelude::*;


fn main() {
    let blah = Arc::new(Mutex::new(Utc::now().timestamp_millis()));

    KeybdKey::bind_all(move |event| {
        match event {
            KeybdKey::LShiftKey => {
                let blah2 = Utc::now().timestamp_millis();
                let mut doot = blah.lock().unwrap();
                println!("{}", blah2 - *doot);

                if blah2 - *doot >= 600 {
                    println!("shifty");
                } else {
                    println!("held shifty");
                }
                *doot = Utc::now().timestamp_millis();
            }
            _ => println!("not shifty")
        }
    });

    // Bind all mouse buttons to a common callback event.
    MouseButton::bind_all(|event| {
        println!("{:?}", event);
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}

