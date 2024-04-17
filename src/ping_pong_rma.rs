use std::ffi::{c_double, c_int, c_void};
use std::mem::size_of;
use std::ops::Add;
use std::ptr;
use mpi::topology::Communicator;
use mpi::{ffi, window};
use mpi::ffi::{MPI_Win, MPI_Win_fence, MPI_Win_free, RSMPI_COMM_WORLD, RSMPI_DOUBLE, RSMPI_INFO_NULL};
use mpi::window::Window;
use mpi_sys::MPI_Aint;

pub fn ping_pong(vector_size: usize, round_num: usize) {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let initiator_rank = 0usize;
    let receiver_rank = 1usize;

    // Start of the main body
    let mut win = Window::allocate(vector_size);

    // **********************
    // * Start of ping pong *
    // **********************
    let mut test_data = Vec::new();
    let t_start = mpi::time();
    // each ping pong repeats 10 times
    for _ in 0..10 {
        win.fence();
        if rank == receiver_rank as i32 {
            win.get_whole_vector(initiator_rank);
        }
        win.fence();
        if rank == receiver_rank as i32 {
            win.window_vector.iter_mut().for_each(|x| *x += 1f64);
            win.put_whole_vector(initiator_rank);
        }
        win.fence();
        if rank == initiator_rank as i32 {
            println!("{:?}", win.window_vector);
        }
    }
    let t_end = mpi::time();
    test_data.push((t_end - t_start) / 10f64 * 1000f64);

    if rank == initiator_rank as i32 {
        println!("Finished {} rounds of ping ping", round_num);
        println!("Obtained {} results", test_data.len());
    }
}
