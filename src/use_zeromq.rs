use std::thread;
use std::time::Duration;

//! Hello World server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "Hello" from client, replies with "World"

fn use_zeromq_responder() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send("World", 0).unwrap();
    }
}

fn use_zeromq_requester() {
    println!("Connecting to hello world server...");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();

    for request_num in 0..10 {
        println!("Sending Hello {}...", request_num);
        requester.send("hello", 0).unwrap();

        requester.recv(&mut msg, 0);
        println!("Receive world {}: {}", msg.as_str().unwrap(), request_num);
    }
}
