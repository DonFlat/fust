use mpi_sys::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

fn main() {
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
