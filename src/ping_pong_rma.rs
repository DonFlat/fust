use mpi::Rank;
use mpi::topology::{Communicator, SimpleCommunicator};
use mpi::window::{AllocatedWindow, WindowOperations};
use crate::test_utils::{append_to_csv, powers_of_two};

pub fn ping_pong(size: u32) {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();

    for n in powers_of_two(size) {
        run_ping_pong(n as usize, rank, &world);
    }
}

fn run_ping_pong(vector_size: usize, rank: Rank, world: &SimpleCommunicator) {
    // *****************
    // * Set up window *
    // *****************
    let mut win: AllocatedWindow<f64> = world.allocate_window(vector_size);
    let mut latency_data = Vec::new();
    // **********************
    // * Start of ping pong *
    // **********************
    for i in 0..12 {
        let t_start = mpi::time();
        win.fence();
        if rank == 1i32 {
            win.get_whole_vector(0);
        }
        win.fence();
        if rank == 1i32 {
            // win.window_vector.iter_mut().for_each(|x| *x += 1f64);
            win.put_whole_vector(0);
        }
        win.fence();
        let t_end = mpi::time();
        // ************************
        // * Collect latency data *
        // ************************
        if rank == 0i32 {
            latency_data.push((t_end - t_start) * 1000000f64);
        }
    }
    if rank == 0i32 {
        append_to_csv("rma_data.csv", vector_size, &latency_data).expect("Failed to write csv");
    }
    if rank == 0 {
        println!("Done with: Rank 0: run ping pong with size: {}", vector_size);
    }
}
