use mpi::Rank;
use mpi::topology::{Communicator, SimpleCommunicator};
use mpi::window::{AllocatedWindow, WindowOperations};
use crate::test_utils::{append_to_csv, generate_test_size};

pub fn ping_pong() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();

    // **********************
    // * Start of ping pong *
    // **********************
    let points = generate_test_size(10, 10_000_000, 100);
    for i in points {
        run_ping_pong(i, rank, &world);
    }
}

fn run_ping_pong(vector_size: usize, rank: Rank, world: &SimpleCommunicator) {
    let mut win: AllocatedWindow<f64> = world.allocate_window(vector_size);
    let mut latency_data = Vec::new();
    // each ping pong repeats 10 times
    for _ in 0..10 {
        let t_start = mpi::time();
        win.fence();
        if rank == 1i32 {
            win.get_whole_vector(0);
        }
        win.fence();
        if rank == 1i32 {
            win.window_vector.iter_mut().for_each(|x| *x += 1f64);
            win.put_whole_vector(0);
        }
        win.fence();
        let t_end = mpi::time();
        // Collect data
        if rank == 0i32 {
            latency_data.push((t_end - t_start) * 1000f64);
        }
    }
    if rank == 0i32 {
        append_to_csv("rma_data.csv", vector_size, &latency_data).expect("Failed to write csv");
    }
}
