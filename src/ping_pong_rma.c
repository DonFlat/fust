#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <mpi.h>

void write_to_csv(double window_size, double* latency) {
    // Open a file for appending
    FILE *fpt = fopen("c_rma.csv", "a");
    if (fpt == NULL) {
        printf("Error opening the file.\n");
        return;
    }

    // Write the window_size as the first column
    fprintf(fpt, "%d", (int)window_size);

    // Write the elements of the latency array as the rest of the columns
    for (int i = 0; i < 11; i++) {
        fprintf(fpt, ",%f", latency[i]);
    }

    // End the line for CSV row
    fprintf(fpt, "\n");

    // Close the file
    fclose(fpt);

    printf("Data appended to output.csv\n");
}

double* powers_of_two(double size) {
    double* result = malloc(size * sizeof(double));
    if (result == NULL) {
        fprintf(stderr, "Memory allocation failed\n");
        return NULL;
    }

    for (int i = 0; i < size; i++) {
        result[i] = pow(2, i);
    }

    return result;
}

void ping_pong(char *argv[], int window_size, int rank) {
    if (rank == 0) {
        printf("window size: %d\n", window_size);
    }

    //  ---- Start RMA
    // Initialize Window
    double *window_base;
    MPI_Win window_handle;

    MPI_Win_allocate(window_size * sizeof(double), sizeof(double), MPI_INFO_NULL,
                     MPI_COMM_WORLD, &window_base, &window_handle);
    // latency data
    double latencies[11];
    // start
    for (int i = 0; i < 11; i++) {
        double start_time = MPI_Wtime();
        MPI_Win_fence(0, window_handle);
        if (rank == 1) {
            MPI_Get(window_base, window_size, MPI_DOUBLE, 0, 0, window_size, MPI_DOUBLE, window_handle);
        }
        MPI_Win_fence(0, window_handle);
        if (rank == 1) {
            for (int i = 0; i < window_size; i++) {
                window_base[i]++;
            }
            MPI_Put(window_base, window_size, MPI_DOUBLE, 0, 0, window_size, MPI_DOUBLE, window_handle);
        }
        MPI_Win_fence(0, window_handle);
        double end_time = MPI_Wtime();
        latencies[i] = (end_time - start_time) * 1000;
    }
    // Print array for debugging
//    if (rank == 0) {
//        printf("\n");
//        for (int i = 0; i < window_size; i++) {
//            printf(" %lf ", window_base[i]);
//        }
//        printf("\n");
//    }
    // Print latency data to csv
    if (rank == 0) {
        write_to_csv(window_size, latencies);
    }

    //  ---- Clean up
    MPI_Win_free(&window_handle);
    return;
}

int main(int argc, char *argv[]) {

    //  ---- Initialize MPI environment
    MPI_Init(&argc, &argv);

    int rank, numprocs;
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &numprocs);

    //  ---- Generate test data
    int size = atoi(argv[1]);
    double* powers = powers_of_two(size);

    for (int i = 0; i < size; i++) {
        ping_pong(argv, powers[i], rank);
    }

    MPI_Finalize();
    return 0;
}