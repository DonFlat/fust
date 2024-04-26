#include <stdio.h>
#include <stdlib.h>
#include <mpi.h>

int main(int argc, char *argv[]) {
    MPI_Init(&argc, &argv);

    int rank, numprocs;
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &numprocs);

    if (numprocs != 2) {
        fprintf(stderr, "This program requires exactly two processes\n");
        MPI_Abort(MPI_COMM_WORLD, 1);
    }

    int window_size = 1624;
    double *window_base;
    MPI_Win window_handle;

    // Allocate memory and create a window simultaneously
    MPI_Win_allocate(window_size * sizeof(double), sizeof(double), MPI_INFO_NULL,
                     MPI_COMM_WORLD, &window_base, &window_handle);

    // Initialize window data on rank 0
    if (rank == 0) {
        for (int i = 0; i < window_size; i++) {
            window_base[i] = 0.0;  // Simple initialization
        }
    }

    // Synchronize before starting RMA operations
    MPI_Win_fence(0, window_handle);

    if (rank == 1) {
        // Perform the Get operation
        MPI_Get(window_base, window_size, MPI_DOUBLE, 0, 0, window_size, MPI_DOUBLE, window_handle);

        // Synchronize again to complete RMA operations
        MPI_Win_fence(0, window_handle);

        // Output the data retrieved by rank 1 (optional, for verification)
        // for (int i = 0; i < window_size; i++) {
        //     printf("Rank 1 got: %f\n", local_data[i]);
        // }
    } else {
        // Rank 0 just participates in the fence
        MPI_Win_fence(0, window_handle);
    }

    // Clean up
    MPI_Win_free(&window_handle);
    MPI_Finalize();
    return 0;
}
