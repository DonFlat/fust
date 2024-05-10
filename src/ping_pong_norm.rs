use mpi::Rank;
use mpi::topology::{Communicator, SimpleCommunicator};
use mpi::traits::*;
use crate::test_utils::{append_to_csv, powers_of_two};

pub fn ping_pong(size: u32) {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();

    for n in powers_of_two(size) {
        run_ping_pong(n as usize, rank, &world);
    }
}

fn run_ping_pong(vec_size: usize, rank: Rank, world: &SimpleCommunicator) {
    let mut het_vec = vec![0f64; vec_size];
    let mut latency_data = Vec::new();
    // **********************
    // * Start of ping pong *
    // **********************
    for _ in 0..11 {
        let t_start = mpi::time();
        if rank == 0 {
            world.process_at_rank(1).send(&het_vec);
        }
        if rank == 1 {
            world
                .process_at_rank(0)
                .receive_into(&mut het_vec);
            // het_vec.iter_mut().for_each(|x| *x += 1f64);
            world.process_at_rank(0).send(&het_vec);
        }
        if rank == 0 {
            world
                .process_at_rank(1)
                .receive_into(&mut het_vec);
        }
        let t_end = mpi::time();
        if rank == 0 {
            latency_data.push((t_end - t_start) * 1000f64);
        }
    }
    if rank == 0 {
        append_to_csv("norm_data.csv", vec_size, &latency_data).expect("Failed to write csv");
    }
}
