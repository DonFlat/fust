extern crate core;

use std::env;
use mpi::topology::Communicator;

mod ping_pong_norm;
mod ping_pong_rma;
mod test_utils;
mod ping_pong_raw_rma;
mod ping_pong_buggy_rma;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mpi_type: &str = &args[1];
    match mpi_type {
        "rma" => ping_pong_rma::ping_pong(),
        "norm" => ping_pong_norm::ping_pong(),
        "raw" => ping_pong_raw_rma::ping_pong(),
        "buggy" => ping_pong_buggy_rma::ping_pong(),
        _ => println!("Invalid argument, run either ping pong | sor_source_data, rma | norm"),
    }
}
