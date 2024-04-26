use std::ffi::{c_int, c_void};
use std::mem::{ManuallyDrop, size_of};
use std::{ptr};
use mpi::ffi::{MPI_Aint, MPI_Win, RSMPI_COMM_WORLD, RSMPI_DOUBLE, RSMPI_INFO_NULL};
use mpi::{ffi, Rank};
use mpi::datatype::Equivalence;
use mpi::raw::AsRaw;
use mpi::topology::{AsCommunicator, Communicator, SimpleCommunicator};
use crate::test_utils::{append_to_csv};

pub fn ping_pong() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();

    // **********************
    // * Start of ping pong *
    // **********************

    // How did I found this bug?
    // I would like to generate a seq of vector_size to test out
    // Generating with log function, see test_util, the sequence is: [10, 21, 43, 89, 183, 379, 785, 1624, 3360, 6952...]
    // But came across seg fault with the sequence
    // Tried to find out which line triggers seg fault, but DAS-6 doesn't have valgrind on node101...
    // SSHed into node101, used module load valgrind, managed to load
    // but once logout, run: prun -np 2 -1 -script $PRUN_ETC/prun-openmpi valgrind `pwd`/./target/debug/pingpong raw
    // It says mpirun unable to run valgrind
    // Thanks to mpirun reports seg fault was from rank 0, therefore use if rank == 0 to print logs
    // Found it is the fence after Get triggers the error.
    // What is even more interesting is just 1624 can lead to seg fault, still haven't found any size else do the same
    // even for 1625. Or very large number
    // Now the problem narrowed down to:
    //   1. Why 1624 makes second fence seg fault?
    //   2. How to run valgrind on DAS?
    run_ping_pong(1624, rank, &world);
}

fn run_ping_pong(vector_size: usize, rank: Rank, world: &SimpleCommunicator) {
    // *****************
    // * Set up window *
    // *****************
    let mut window_base: *mut f64 = ptr::null_mut();
    let mut window_handle: MPI_Win = ptr::null_mut();

    unsafe {
        ffi::MPI_Win_allocate(
            (vector_size * size_of::<f64>()) as MPI_Aint,
            size_of::<f64>() as c_int,
            RSMPI_INFO_NULL,
            world.as_communicator().as_raw(),
            &mut window_base as *mut *mut _ as *mut c_void,
            &mut window_handle
        );
    }
    let mut window_vector = ManuallyDrop::new(
        unsafe {
            Vec::from_raw_parts(window_base, vector_size, vector_size)
        }
    );

    let mut latency_data = Vec::new();
    // *******************
    // * Start Ping-Pong *
    // *******************
        let t_start = mpi::time();
        unsafe {
            ffi::MPI_Win_fence(0, window_handle);
        }
        if rank == 1i32 {
            unsafe {
                ffi::MPI_Get(
                    window_base as *mut c_void,
                    window_vector.len() as c_int,
                    RSMPI_DOUBLE,
                    0,
                    0,
                    window_vector.len() as c_int,
                    RSMPI_DOUBLE,
                    window_handle
                );
            }
        }
        println!("rank: {}, before second fence, window_vec len: {}", rank, window_vector.len());
        unsafe {
            ffi::MPI_Win_fence(0, window_handle);
        }
        println!("rank: {}, after second fence", rank);
        if rank == 1i32 {
            window_vector.iter_mut().for_each(|x| *x += 1f64);
            unsafe {
                ffi::MPI_Put(
                    window_base as *mut c_void,
                    window_vector.len() as c_int,
                    f64::equivalent_datatype().as_raw(),
                    0,
                    0,
                    window_vector.len() as c_int,
                    f64::equivalent_datatype().as_raw(),
                    window_handle
                );
            }
        }
        unsafe {
            ffi::MPI_Win_fence(0, window_handle);
        }
        let t_end = mpi::time();
        // ************************
        // * Collect latency data *
        // ************************
        if rank == 0i32 {
            latency_data.push((t_end - t_start) * 1000f64);
            // println!("{:?}", window_vector);
        }

    unsafe {
        ffi::MPI_Win_free(&mut window_handle);
    }

    if rank == 0i32 {
        append_to_csv("raw_data.csv", vector_size, &latency_data).expect("Failed to write csv");
    }
}
