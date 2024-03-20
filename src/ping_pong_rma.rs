use mpi_sys::*;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::mem::size_of;
use mpi::topology::Communicator;

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
    let mut het_vec = vec![0; vector_size];
    // Create a window, not allocate
    // create is you already have an allocated buffer
    // allocate is you haven't, MPI allocate it for you

    // Displacement unit: simplify access with a single datatype
    // typical use: either 1 (all access are in terms of byte offset) or sizeof(type)
    let mut window = ptr::null_mut();
    unsafe {
        MPI_Win_create(
            het_vec.as_mut_ptr() as *mut c_void,
            (vector_size * size_of::<c_int>()) as MPI_Aint,
            size_of::<c_int>() as c_int,
            RSMPI_INFO_NULL,
            RSMPI_COMM_WORLD,
            &mut window
        );
    }

    // **********************
    // * Start of ping pong *
    // **********************
    let t_start = mpi::time();
    for round in 0..round_num {

        // Start of a epoch with fence
        unsafe {
            MPI_Win_fence(0, window);
        }

        if rank == receiver_rank {
            //println!("Initiator rank {} starts ping", initiator_rank);
            unsafe {
                MPI_Get(
                    het_vec.as_mut_ptr() as *mut c_void,
                    vector_size as c_int,
                    RSMPI_INT32_T,
                    initiator_rank,
                    0,
                    vector_size as c_int,
                    RSMPI_INT32_T,
                    window
                );
            }
        }

        // unsafe {
        //     MPI_Win_fence(0, window);
        // }

        if rank == receiver_rank {
            het_vec.iter_mut().for_each(|x| *x += 1);
            unsafe {
                MPI_Put(
                    het_vec.as_mut_ptr() as *mut c_void,
                    vector_size as c_int,
                    RSMPI_INT32_T,
                    initiator_rank,
                    0,
                    vector_size as c_int,
                    RSMPI_INT32_T,
                    window
                );
            }
        }

        unsafe { MPI_Win_fence(0, window); }
    }
    let t_end = mpi::time();
    if rank == initiator_rank {
        println!("--- vec: {:?} ---", het_vec);
        println!("Finished {} rounds of ping ping, time: {}", round_num, t_end - t_start);
    }

    // should release window as it is not automatic
    unsafe {
        MPI_Win_free(&mut window);
    }
}
