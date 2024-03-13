use std::f64::consts::PI;
use mpi::collective::SystemOperation;
use mpi::traits::*;

fn stencil(matrix: &Vec<Vec<f64>>, row: usize, col: usize) -> f64 {
    let direction = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let sum = direction.iter().fold(0.0, |acc, (r_off, c_off)| {
        let r = (row as isize + r_off) as usize;
        let c = (col as isize + c_off) as usize;
        acc + matrix[r][c]
    });
    sum / 4.0
}

fn even_1_odd_0(num: usize) -> usize {
    match num % 2 {
        0 => 1,
        _ => 0
    }
}

// Suppose n = 1000, size = 4, rank = 2
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

pub fn sor() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let pred_rank = if rank == 0 { 0 } else { rank - 1 };
    let succ_rank = if rank == size - 1 { rank } else { rank + 1 };

    // initialize the basic variables
    let mut N = 1000;
    if rank == 0 {
        println!("Running SOR on {} nodes with {} rows", size, N);
    }
    N += 2;

    let n_col = N;
    let n_row = N;
    let r = 0.5 * ((PI / n_col as f64).cos() + (PI / n_row as f64).cos());
    let mut  omega =  2.0 / (1.0 + (1.0 - r * r).sqrt());
    let stop_diff = 0.0002 / (2.0 - omega);
    let mut max_diff;
    let mut diff;
    omega *= 0.8;

    // get my stripe bounds and malloc the grid accordingly
    let (mut global_lb, mut global_ub) = get_bounds(N - 1, size as usize, rank as usize);
    // row 0 is static
    if global_lb == 0 {
        global_lb = 1;
    }
    // Initialize the matrix at local rank, full size, 0 filled
    let local_ub = global_ub - (global_lb - 1) + 1;
    let mut matrix = vec![vec![0.0; n_col]; local_ub];
    // Initialize the boundary value
    for i in 0..=local_ub {
        for j in 0..n_col {
            matrix[i][j] = if i == 0 {
                4.56
            } else if i == n_row - 1 {
                9.85
            } else if j == 0 {
                7.32
            } else if j == n_col - 1 {
                6.88
            } else {
                0.0
            }
        }
    }

    let t_start = mpi::time();
    // Now do the real computation
    let mut iteration = 0;
    loop {
        // TODO: [lb-1, ub] was initialized
        // TODO: send row 1 to pred rank; send second last row to succ
        // TODO: receive into row 0; receive into last row
        world.process_at_rank(pred_rank).send_with_tag(&matrix[1], 42);
        world.process_at_rank(succ_rank).send_with_tag(&matrix[local_ub -1], 42);
        world.process_at_rank(pred_rank).receive_into_with_tag(&mut matrix[0], 42);
        world.process_at_rank(succ_rank).receive_into_with_tag(&mut matrix[local_ub], 42);
        max_diff = 0.0;
        for phase in 0..2 {
            // TODO: row [second row, last row)
            for i in 1..local_ub {
                let start_col = 1 + (even_1_odd_0(i) ^ phase);
                for j in (start_col..n_col-1).step_by(2) {
                    let stencil_val = stencil(&matrix, i, j);
                    diff = (stencil_val - matrix[i][j]).abs();
                    if diff > max_diff {
                        max_diff = diff;
                    }
                    matrix[i][j] = matrix[i][j] + omega * (stencil_val - matrix[i][j]);
                }
            }
        }
        diff = max_diff;
        world.all_reduce_into(&diff, &mut max_diff, SystemOperation::max());
        iteration += 1;

        if max_diff <= stop_diff {
            break;
        }
    }
    let t_end = mpi::time();

    if rank == 0 {
        println!("SOR {} x {} took {} s", n_row-2, n_col-2,t_end-t_start);
        println!("using {} iterations, diff is {} (allowed diff {})", iteration,max_diff,stop_diff)
    }
}

// try from 10x10, check matrix if identical
// iterations should be same
// only use boundary rows
// check out if optimization options