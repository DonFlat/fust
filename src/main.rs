use std::env;
use crate::ping_pong_rma::ping_pong;

mod use_mpi;
mod use_zeromq;
mod ping_pong_rma;
mod sor_seq;
mod sor;
mod sor_rma;
mod ping_pong_norm;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem_or_vec_size = args[1].parse().expect("Failed to parse args[1] as usize");
    let iteration_or_ping_rounds = args[2].parse().expect("Failed to parse args[2] as usize");
    let mpi_type: &str = &args[3];
    match mpi_type {
        "norm" => sor::sor(problem_or_vec_size, iteration_or_ping_rounds),
        "rma" => sor_rma::sor(problem_or_vec_size, iteration_or_ping_rounds),
        "pin_rma" => ping_pong_rma::ping_pong(problem_or_vec_size, iteration_or_ping_rounds),
        "pin_norm" => ping_pong_norm::ping_pong(problem_or_vec_size, iteration_or_ping_rounds),
        "seq" => sor_seq::sor(problem_or_vec_size, iteration_or_ping_rounds),
        _ => println!("Invalid argument, run either ping pong | sor, rma | norm")
    }
}
