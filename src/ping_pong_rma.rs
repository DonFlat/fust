use mpi_sys::*;
use mpi::topology::Communicator;
use mpi::window;

pub fn ping_pong(vector_size: usize, round_num: usize) {

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();


    // exception handling
    if size != 2 {
        println!("World size is {}, 2 is expected", size);
        unsafe { MPI_Abort(RSMPI_COMM_WORLD, 1); }
    }

    let initiator_rank = 0;
    let receiver_rank = 1;

    // Start of the main body
    let mut handle = window::Window::new(vector_size);

    // **********************
    // * Start of ping pong *
    // **********************
    let mut test_data = Vec::new();
    // Grow size by 5
    for message_size in (1..=vector_size).step_by(5) {
        let t_start = mpi::time();
        // each ping pong repeats 10 times
        for _ in 0..10 {
            handle.fence();
            if rank == receiver_rank {
                handle.get_whole_vector(initiator_rank);
            }
            handle.fence();
            if rank == receiver_rank {
                handle.put_whole_vector(initiator_rank);
            }
            handle.fence();
        }
        let t_end = mpi::time();
        test_data.push((t_end - t_start) / 10f64 * 1000f64);
    }

    if rank == initiator_rank as i32 {
        println!("Finished {} rounds of ping ping", round_num);
        println!("Obtained {} results", test_data.len());
    }
}
