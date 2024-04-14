use std::ffi::{c_double, c_int, c_void};
use std::mem::size_of;
use std::ops::Add;
use std::ptr;
use mpi::topology::Communicator;
use mpi::ffi;
use mpi::ffi::{MPI_Win, MPI_Win_fence, MPI_Win_free, RSMPI_COMM_WORLD, RSMPI_DOUBLE, RSMPI_INFO_NULL};
use mpi_sys::MPI_Aint;

pub fn ping_pong(vector_size: usize, round_num: usize) {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let initiator_rank = 0usize;
    let receiver_rank = 1usize;

    // Start of the main body
    let mut window_base: *mut f64 = ptr::null_mut();
    let mut window_handle: MPI_Win = ptr::null_mut();
    if rank == initiator_rank as  i32 {
        println!("{:p}", window_base);
    }
    unsafe {
        ffi::MPI_Win_allocate(
            (vector_size * size_of::<c_double>()) as MPI_Aint,
            size_of::<c_double>() as c_int,
            RSMPI_INFO_NULL,
            RSMPI_COMM_WORLD,
            &mut window_base as *mut *mut f64 as *mut c_void, // Correctly pass a pointer to a pointer
            &mut window_handle // Correctly pass a pointer to MPI_Win
        );
    }

    // **********************
    // * Start of ping pong *
    // **********************
    let mut test_data = Vec::new();
    let t_start = mpi::time();
    // each ping pong repeats 10 times
    for _ in 0..10 {
        unsafe { MPI_Win_fence(0, window_handle); }
        if rank == receiver_rank as i32 {
            unsafe {
                ffi::MPI_Get(
                    window_base as *mut c_void,
                    vector_size as c_int,
                    RSMPI_DOUBLE,
                    initiator_rank as c_int,
                    0,
                    vector_size as c_int,
                    RSMPI_DOUBLE,
                    window_handle
                );
            }
        }
        unsafe { MPI_Win_fence(0, window_handle); }
        if rank == receiver_rank as i32 {
            unsafe {
                for i in 0..vector_size {
                    *window_base.add(i) += 1f64;
                }
                ffi::MPI_Put(
                    window_base as *mut c_void,
                    vector_size as c_int,
                    RSMPI_DOUBLE,
                    initiator_rank as c_int,
                    0,
                    vector_size as c_int,
                    RSMPI_DOUBLE,
                    window_handle
                );
            }
        }
        unsafe { MPI_Win_fence(0, window_handle); }
        if rank == initiator_rank as i32 {
            if rank == initiator_rank as  i32 {
                println!("{:p}", window_base);
            }
            for i in 0..vector_size {
                unsafe {
                    print!("{}, ", *window_base.add(i));
                }
            }
            println!();
        }
    }
    let t_end = mpi::time();
    test_data.push((t_end - t_start) / 10f64 * 1000f64);

    if rank == initiator_rank as i32 {
        println!("Finished {} rounds of ping ping", round_num);
        println!("Obtained {} results", test_data.len());
    }

    unsafe {
        MPI_Win_free(&mut window_handle);
    }
}
