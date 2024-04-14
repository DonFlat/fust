use std::env;
mod ping_pong_norm;
mod ping_pong_rma;

fn main() {
    let args: Vec<String> = env::args().collect();
    let vec_size = args[1].parse().expect("Failed to parse args[1] as usize");
    let ping_rounds = args[2].parse().expect("Failed to parse args[2] as usize");
    let mpi_type: &str = &args[3];
    match mpi_type {
        "pin_rma" => ping_pong_rma::ping_pong(vec_size, ping_rounds),
        "pin_norm" => ping_pong_norm::ping_pong(vec_size, ping_rounds),
        _ => println!("Invalid argument, run either ping pong | sor_source_data, rma | norm"),
    }
}
