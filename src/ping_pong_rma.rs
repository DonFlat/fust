use mpi::topology::Communicator;
use mpi::window::Window;
use mpi_sys::*;

pub fn ping_pong(vector_size: usize, round_num: usize) {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let initiator_rank = 0usize;
    let receiver_rank = 1usize;

    // Start of the main body
    let mut handle = Window::new(vector_size);

    // **********************
    // * Start of ping pong *
    // **********************
    let mut test_data = Vec::new();
    let t_start = mpi::time();
    // each ping pong repeats 10 times
    for _ in 0..10 {
        handle.fence();
        if rank == receiver_rank as i32 {
            handle.get_whole_vector(initiator_rank);
        }
        handle.fence();
        if rank == receiver_rank as i32 {
            handle.window_vector.iter_mut().for_each(|x| *x += 1f64);
            handle.put_whole_vector(initiator_rank);
        }
        handle.fence();
        if rank == initiator_rank as i32 {
            println!("vector content: {:?}", handle.window_vector);
        }
    }
    let t_end = mpi::time();
    test_data.push((t_end - t_start) / 10f64 * 1000f64);

    if rank == initiator_rank as i32 {
        println!("Finished {} rounds of ping ping", round_num);
        println!("Obtained {} results", test_data.len());
    }
}
