use std::env;

mod use_mpi;
mod use_zeromq;
mod ping_pong_mpi;
mod sor_seq;
mod sor;
mod sor_rma;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem_size = args[1].parse().expect("Failed to parse args[1] as usize");
    let iteration = args[2].parse().expect("Failed to parse args[2] as usize");
    let mpi_type: &str = &args[3];
    match mpi_type {
        "norm" => sor::sor(problem_size, iteration),
        "rma" => sor_rma::sor(problem_size, iteration),
        _ => sor_seq::sor(problem_size, iteration)
    }
}
