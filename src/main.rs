use std::env;
use mpi::topology::Communicator;

mod ping_pong_norm;
mod ping_pong_rma;
mod test_utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let vec_size = args[1].parse().expect("Failed to parse args[1] as usize");
    // let ping_rounds = args[2].parse().expect("Failed to parse args[2] as usize");
    let mpi_type: &str = &args[1];
    match mpi_type {
        "rma" => ping_pong_rma::ping_pong(),
        "norm" => ping_pong_norm::ping_pong(0, 0),
        _ => println!("Invalid argument, run either ping pong | sor_source_data, rma | norm"),
    }
}
