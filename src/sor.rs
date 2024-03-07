use std::f64::consts::PI;
use mpi::collective::SystemOperation;
use mpi::traits::*;

fn stencil(matrix: &Vec<Option<Vec<f64>>>, row: usize, col: usize) -> f64 {
    // 0.0 is magic number.
    let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // Up, Down, Left, Right

    let values = offsets.iter().map(|&(r_off, c_off)| {
        let new_row = row as isize + r_off; // Calculating new row index
        let new_col = col as isize + c_off; // Calculating new column index

        matrix.get(new_row as usize)  // Safely getting the row
            .and_then(|r| r.as_ref()) // Handling Option
            .and_then(|current_row| current_row.get(new_col as usize)) // Safely getting the column
            .and_then(|&cell| Some(cell)) // Extracting value
            .unwrap_or_else(|| panic!("No value at position ({}, {})", new_row, new_col)) // Handling missing value
    });

    values.sum::<f64>() / 4.0
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
    // if N < size as usize {
    //     N = size as usize;
    // }
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
    let (lower_bound, upper_bound) = get_bounds(N - 1, size as usize, rank as usize);
    let mut lb = lower_bound;
    let mut ub = upper_bound;
    // row 0 is static
    if lb == 0 {
        lb = 1;
    }
    // Initialize the matrix at local rank, full size, 0 filled
    let mut matrix: Vec<Option<Vec<f64>>> = Vec::with_capacity(n_row);
    // [0, lb-1) are none
    for _ in 0..lb-1 {
        matrix.push(None);
    }
    // [lb-1, ub] are some
    for _ in lb-1..=ub {
        matrix.push(Some(vec![0.0; n_col]));
    }
    // (ub + 1, n_row) are none
    for _ in ub+1..n_row {
        matrix.push(None);
    }
    // Initialize the boundary value
    for i in lb-1..=ub {
        for j in 0..n_col {
            if let Some(current_row) = &mut matrix[i] {
                current_row[j] = if i == 0 {
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
    }

    let t_start = mpi::time();
    // Now do the real computation
    let mut iteration = 0;
    loop {
        if let Some(row_lb) = &matrix[lb] {
            world.process_at_rank(pred_rank).send_with_tag(row_lb, 42);
        }
        if let Some(row_ub_1) = &matrix[ub-1] {
            world.process_at_rank(succ_rank).send_with_tag(row_ub_1, 42);
        }
        if let Some(row_lb_1) = &mut matrix[lb - 1] {
            world.process_at_rank(pred_rank).receive_into_with_tag(row_lb_1, 42);
        }
        if let Some(row_ub) = &mut matrix[ub] {
            world.process_at_rank(succ_rank).receive_into_with_tag(row_ub, 42);
        }

        max_diff = 0.0;
        for phase in 0..2 {
            for i in lb..ub {
                let start_col = 1 + (even_1_odd_0(i) ^ phase);
                for j in (start_col..n_col-1).step_by(2) {
                    let stencil_val = stencil(&matrix, i, j);
                    if let Some(current_row) = &mut matrix[i] {
                        diff = (stencil_val - current_row[j]).abs();
                        if diff > max_diff {
                            max_diff = diff;
                        }
                        current_row[j] = current_row[j] + omega * (stencil_val - current_row[j]);
                    }
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