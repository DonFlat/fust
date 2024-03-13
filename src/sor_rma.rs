use std::f64::consts::PI;
use std::ffi::{c_double, c_void};
use std::mem::size_of;
use std::os::raw::c_int;
use std::ptr;
use mpi::traits::*;
use mpi_sys::*;

// TODO: how would you tell it is a contigious memory block, or array of pointers?
fn get_bounds(n: usize, size: usize, rank: usize) -> (usize, usize) {
    let mut nlarge = n % size; // 1000 % 4 = 0
    let mut nsmall = size - nlarge; // 4 - 0 = 4

    let mut size_small = n / size; // 1000 / 4 = 25
    let  size_large = size_small + 1; // 25 + 1 = 26

    let mut lower_bound;
    let mut upper_bound;

    if rank < nlarge { // 2 < 0 ?
        lower_bound = rank * size_large;
        upper_bound = lower_bound + size_large;
    } else {
        // 0 * 26 + (2 - 0) * 4 = 8
        lower_bound = nlarge * size_large + (rank - nlarge) * size_small;
        // 8 + 25 = 33
        upper_bound = lower_bound + size_small;
    }
    (lower_bound, upper_bound)
}

// break down of the task:
// 1. MPI initialization
// 2. initialize the sub matrix:
//      what was sent: G[lb], G[ub-1]. What was received G[lb-1], G[ub]
// 3. initializing window for matrix:
//      each process have an open window:
//          size is # element in sub matrix
//  Globally [lb-1, ub] have value, so ub-(lb-1)+1 rows, locally it is [0, ub-(lb-1)]
// 4. window for diff & max_diff
pub unsafe fn sor() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let pred_rank = if rank == 0 { 0 } else { rank - 1 };
    let succ_rank = if rank == size - 1 { rank } else { rank + 1 };

    let mut N = 1000;
    if rank == 0 {
        println!("Running RMA SOR on {} nodes with {} rows", size, N);
    }
    N += 2;

    let n_col = N;
    let n_row = N;
    let r = 0.5 * ((PI / n_col as f64).cos() + (PI / n_row as f64).cos());
    let mut  omega =  2.0 / (1.0 + (1.0 - r * r).sqrt());
    let stop_diff = 0.0002 / (2.0 - omega);
    // let mut max_diff;
    // let mut diff;
    omega *= 0.8;

    // get my stripe bounds and malloc the grid accordingly
    let (lower_bound, upper_bound) = get_bounds(N - 1, size as usize, rank as usize);
    let mut lb = lower_bound;
    let mut ub = upper_bound;
    // row 0 is static
    if lb == 0 {
        lb = 1;
    }

    // Initialization of Window in this process
    // local matrix: ub-(lb-1)+1 rows, n_col columns
    let elem_number = (ub-(lb-1)+1) * n_col;
    let mut local_matrix = vec![0.0; elem_number];
    let mut  window= ptr::null_mut();
    unsafe {
        MPI_Win_create(
            local_matrix.as_mut_ptr() as *mut c_void,
            (elem_number * size_of::<c_double>()) as MPI_Aint,
            size_of::<c_double>() as c_int,
            RSMPI_INFO_NULL,
            RSMPI_COMM_WORLD,
            &mut window
        );
    }
    let mut matrices = vec![vec![114.514; n_col]; n_row];
    if rank == 0 {
        unsafe {
            let mut m = matrices.as_mut_ptr();
            println!("After manual shift: {:?}", *m.add(1001));
        }
        println!("The matrices content: {}", matrices[0][1]);
    }
    for i in 0..elem_number {

    }
}