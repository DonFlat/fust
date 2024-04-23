use std::ffi::{c_int, c_void};
use std::mem::{ManuallyDrop, size_of};
use std::{ptr};
use mpi::ffi::{MPI_Aint, MPI_Win, RSMPI_INFO_NULL};
use mpi::{ffi, Rank};
use mpi::datatype::Equivalence;
use mpi::raw::AsRaw;
use mpi::topology::{AsCommunicator, Communicator, SimpleCommunicator};
use mpi::window::{AllocatedWindow, WindowOperations};
use crate::test_utils::{append_to_csv, generate_test_size};

pub fn ping_pong() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();

    // **********************
    // * Start of ping pong *
    // **********************
    //let points = generate_test_size(10, 10_000_000, 100);
    //for i in points {
        run_ping_pong(10, rank, &world);
    //}
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
    // **********************
    // * Start Ping-Pong    *
    // **********************
    for i in 0..10 {
        let t_start = mpi::time();
        // println!("Rank: {}_Time: {}_repetition: {}_vecsize: {}, before get", rank, mpi::time() * 100f64, i, vector_size);
        unsafe {
            ffi::MPI_Win_fence(0, window_handle);
        }
        if rank == 1i32 {
            unsafe {
                ffi::MPI_Get(
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
        // println!("Rank: {}_Time: {}_repetition: {}_vecsize: {}, before put", rank, mpi::time() * 100f64, i, vector_size);
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
        // println!("Rank: {}_Time: {}_repetition: {}_vecsize: {}, a round done", rank, mpi::time() * 100f64, i, vector_size);
        // Collect data
        if rank == 0i32 {
            latency_data.push((t_end - t_start) * 1000f64);
            // println!("{:?}", window_vector);
        }
    }
    if rank == 0i32 {
        append_to_csv("raw_data.csv", vector_size, &latency_data).expect("Failed to write csv");
    }
}
