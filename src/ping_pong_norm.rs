use mpi::traits::*;
use mpi::topology::Communicator;

pub fn ping_pong(vector_size: usize, round_num: usize) {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let initiator_rank = 0;
    let receiver_rank = 1;

    let mut het_vec = vec![0; vector_size];

    // **********************
    // * Start of ping pong *
    // **********************
    let t_start = mpi::time();
    for i in 0..round_num {
        if rank == initiator_rank {
            // println!("=== Start round {} ===", i);
            world.process_at_rank(receiver_rank).send(&het_vec);
        }
        if rank == receiver_rank {
            world.process_at_rank(initiator_rank).receive_into(&mut het_vec);
            het_vec.iter_mut().for_each(|x| *x += 1);
            world.process_at_rank(initiator_rank).send(&het_vec);
        }
        if rank == initiator_rank {
            world.process_at_rank(receiver_rank).receive_into(&mut het_vec);
            // println!("--- round {} done, vec: {:?}", i, het_vec);
        }
    }
    let t_end = mpi::time();
    if rank == initiator_rank {
        println!("Finished {} rounds of ping pong, time: {}", round_num, t_end - t_start);
    }
}