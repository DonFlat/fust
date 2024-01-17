mod use_mpi;
mod use_zeromq;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "mpi" => use_mpi::use_mpi(),
        "zmq" => {
            match args[2].as_str() {
                "res" => use_zeromq::use_zeromq_responder(),
                "req" => use_zeromq::use_zeromq_requester(),
                _ => println!("invalid")
            }
        },
        _ => println!("invalid")
    }
}
