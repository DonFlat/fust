use mpi_sys::*;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::mem::size_of;
use std::time::Instant;

pub fn ping_pong() {
    // Initialize
    unsafe { MPI_Init(ptr::null_mut(), ptr::null_mut()); }
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
    let mut master_buf= [0; 5].as_mut_ptr();
    let mut my_buf = [0; 5].as_mut_ptr();
    // Create a window, not allocate
    // create is you already have an allocated buffer
    // allocate is you haven't, MPI allocate it for you

    // Displacement unit: simplify access with a single datatype
    // typical use: either 1 (all access are in terms of byte offset) or sizeof(type)
    let mut window = ptr::null_mut();
    if rank == 0 {
        unsafe { MPI_Win_create(master_buf as *mut c_void, (5 * size_of::<c_int>()) as MPI_Aint,
                                size_of::<c_int>() as c_int, RSMPI_INFO_NULL, RSMPI_COMM_WORLD, &mut window);
        }
    } else {
        unsafe { MPI_Win_create(ptr::null_mut(), 0, 1,
                                RSMPI_INFO_NULL, RSMPI_COMM_WORLD, &mut window);
        }
    }

    if rank == 0 {
        for i in 0..5 {
            unsafe { *master_buf.add(i) = 114514 as c_int };
        }
    }

    let sender_rank = 0;
    let receiver_rank = 1;

    // Start of a epoch with fence
    unsafe { MPI_Win_fence(0, window); }

    if rank == receiver_rank {
        unsafe { MPI_Get(my_buf as *mut c_void, 5, RSMPI_INT32_T,
                         0, 0, 5, RSMPI_INT32_T, window); }
    }

    unsafe { MPI_Win_fence(0, window); }

    if rank == 1 {
        for i in 0..5 {
            unsafe {
                let val = *(my_buf.wrapping_add(i));
                println!("{}", val);
            }
        }
    }

    // Clean up and finish
    unsafe {
        MPI_Win_free(&mut window);
        MPI_Finalize();
    }
}