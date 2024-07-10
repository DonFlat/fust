#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <mpi.h>

void write_to_csv(double window_size, double* latency) {
    // Open a file for appending
    FILE *fpt = fopen("c_norm.csv", "a");
    if (fpt == NULL) {
        printf("Error opening the file.\n");
        return;
    }

    // Write the window_size as the first column
    fprintf(fpt, "%d", (int)window_size);

    // Write the elements of the latency array as the rest of the columns
    for (int i = 0; i < 12; i++) {
        fprintf(fpt, ",%f", latency[i]);
    }

    // End the line for CSV row
    fprintf(fpt, "\n");

    // Close the file
    fclose(fpt);

    printf("Data appended to c_norm.csv\n");
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

void ping_pong(int array_size, int rank) {
    // Initialize array
    double data[array_size];
    for (int i = 0; i < array_size; i++) {
        data[i] = 0;
    }
    // latency data
    double latencies[12];
    // start
    for (int i = 0; i < 12; i++) {
        double start_time = MPI_Wtime();

        if (rank == 0) {
            MPI_Send(data, array_size, MPI_DOUBLE, 1, 0, MPI_COMM_WORLD);

//            printf("\n");
//            for (int i = 0; i < array_size; i++) {
//                printf(" %lf ", data[i]);
//            }
//            printf("\n");

            MPI_Recv(data, array_size, MPI_DOUBLE, 1, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
        } else if (rank == 1) {
            MPI_Recv(data, array_size, MPI_DOUBLE, 0, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);

            for (int i = 0; i < array_size; i++) {
                data[i]++;
            }

            MPI_Send(data, array_size, MPI_DOUBLE, 0, 0, MPI_COMM_WORLD);
        }

        double end_time = MPI_Wtime();
        latencies[i] = (end_time - start_time) * 1000000;
    }
    // Print latency data to csv
    if (rank == 0) {
        write_to_csv(array_size, latencies);
    }

    if (rank == 0) {
        printf("Done with vector size: %d\n", array_size);
    }
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
        ping_pong(powers[i], rank);
    }

    MPI_Finalize();
    return 0;
}