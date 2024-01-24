use mpi_sys::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

pub fn use_mpi() {
    unsafe {
        MPI_Init(std::ptr::null_mut(), std::ptr::null_mut());
    }

    let mut world_size: c_int = 0;
    unsafe {
        MPI_Comm_size(RSMPI_COMM_WORLD, &mut world_size);
    }

    let mut world_rank: c_int = 0;
    unsafe {
        MPI_Comm_rank(RSMPI_COMM_WORLD, &mut world_rank);
    }

    let mut name = vec![0 as c_char; MPI_MAX_PROCESSOR_NAME as usize];

    let mut result_len: c_int = 0;

    unsafe {
        MPI_Get_processor_name(name.as_mut_ptr(), &mut result_len);
    }

    let name = unsafe { CStr::from_ptr(name.as_ptr()) };
    let processor_name = name.to_string_lossy();

    println!(
        "Hello from process {} of {}, on processor: {}",
        world_rank, world_size, processor_name
    );

    unsafe {
        MPI_Finalize();
    }
}

pub fn try_rma() {
    unsafe {
        // Initialize MPI
        MPI_Init(ptr::null_mut(), ptr::null_mut());

        // Get the rank and size
        let mut rank: c_int = 0;
        let mut size: c_int = 0;
        MPI_Comm_rank(RSMPI_COMM_WORLD, &mut rank);
        MPI_Comm_size(RSMPI_COMM_WORLD, &mut size);

        if size < 2 {
            eprintln!("This program requires at least two processes");
            MPI_Abort(RSMPI_COMM_WORLD, 1);
        }

        let mut win: MPI_Win = ptr::null_mut();
        let mut data: c_int = if rank == 0 { 123 } else { 0 };

        // Create an MPI window
        MPI_Win_create(
            &mut data as *mut _ as *mut c_void,
            std::mem::size_of::<c_int>() as MPI_Aint,
            std::mem::size_of::<c_int>() as c_int,
            RSMPI_INFO_NULL,
            RSMPI_COMM_WORLD,
            &mut win,
        );

        // Synchronize
        MPI_Win_fence(0, win);

        if rank == 0 {
            // Rank 0 sends data to Rank 1
            let target_rank = 1;
            MPI_Put(
                &data as *const _ as *const c_void,
                1,
                RSMPI_INT32_T,
                target_rank,
                0,
                1,
                RSMPI_INT32_T,
                win,
            );
        }

        // Another synchronization
        MPI_Win_fence(0, win);

        if rank == 1 {
            // Rank 1 prints the received data
            println!("Rank 1 received data: {}", data);
        }

        // Clean up
        MPI_Win_free(&mut win);
        MPI_Finalize();
    }
}