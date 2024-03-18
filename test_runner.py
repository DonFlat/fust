import subprocess
import csv

# Define the problem sizes
problem_sizes = [10, 50, 100, 200, 400]
mpi_types = ["norm", "rma"]

# Initialize a list to hold the rows of the CSV file
csv_rows = []
proc_number = 4

# Add the header row
header_row = ['Problem Size'] + mpi_types
csv_rows.append(header_row)

# Run the Rust programs and collect the results
for problem_size in problem_sizes:
    current_row = [problem_size]  # Start the row with the problem size
    for mpi_type in mpi_types:
        command = f"mpirun -n {proc_number} ./target/debug/fust {problem_size} {mpi_type}"
        try:
            # Run the command
            print(f"Running {command}")
            output = subprocess.check_output(command, shell=True, text=True)

            # Extract the time from the output
            time_str = output.split("time: ")[1].split(" ")[0]
            time_float = float(time_str)

            # Append the time to the current row
            current_row.append(time_float)
        except subprocess.CalledProcessError as e:
            print(f"Error running command '{command}': {e}")
            current_row.append('Error')  # Use 'Error' or None as appropriate

    # Add the current row to the list of rows for the CSV
    csv_rows.append(current_row)

# Write the results to a CSV file
with open(f'mpi_rust_results_proc_{proc_number}.csv', 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerows(csv_rows)

print("CSV file has been generated.")
