use mpi_sys::*;
use std::os::raw::{c_int, c_void};
use std::{mem};
use std::time::Instant;

pub fn ping_pong() {
    // Initialize
    unsafe { MPI_Init(std::ptr::null_mut(), std::ptr::null_mut()); }
    // Get world size and rank
    let mut world_size: c_int = 0;
    let mut rank: c_int = 0;

    unsafe {
        MPI_Comm_rank(RSMPI_COMM_WORLD, &mut rank);
        MPI_Comm_size(RSMPI_COMM_WORLD, &mut world_size);
    }

    // exception handling
    if world_size != 2 {
        println!("World size is {}, 2 is expected", world_size);
        unsafe { MPI_Abort(RSMPI_COMM_WORLD, 1); }
    }

    // Start of the main body

    let mut round = 0;
    // Declaring a window type
    // let mut window: MPI_Win = unsafe { mem::MaybeUninit::uninit().assume_init() };
    let mut window = std::ptr::null_mut();
    // Create a window, not allocate
    // where you already have a local buffer to expose
    unsafe {
        MPI_Win_create(
            &mut round as *mut c_int as *mut c_void,
            mem::size_of::<c_int>() as MPI_Aint,
            1,
            RSMPI_INFO_NULL,
            RSMPI_COMM_WORLD,
            &mut window
        );
    }

    // Start of ping pong
    for i in 0..10 {
        // Start of a epoch with fence
        unsafe { MPI_Win_fence(0, window); }

        let data = 1;
        let sender_rank = i % 2;
        let receiver_rank = 1 - i % 2;

        let start = Instant::now();
        if rank == i % 2 {
            // send with put
            unsafe {
                MPI_Put(
                    &data as *const c_int as *const c_void,
                    1,
                    RSMPI_INT32_T,
                    receiver_rank,
                    0,
                    1,
                    RSMPI_INT32_T,
                    window
                );
            };
            // println!("i = {}, rank {} -> {} ", i, sender_rank, receiver_rank);
        }
        // end of a epoch with fence
        unsafe { MPI_Win_fence(0, window); }

        if rank != i % 2 {
            let duration = start.elapsed().as_secs_f64() * 1000.0;
            println!("sender: {}, receiver: {}, latency: {}", sender_rank, receiver_rank, duration);
        }
    }

    // Clean up and finish
    unsafe {
        MPI_Win_free(&mut window);
        MPI_Finalize();
    }

}