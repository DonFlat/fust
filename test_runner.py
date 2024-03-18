import subprocess
import pandas as pd
import matplotlib.pyplot as plt

# Define the problem sizes
problem_sizes = [10, 50, 100, 500, 1000, 1500, 2000]
mpi_types = ["norm", "rma"]

# Initialize a dictionary to hold the results, with keys for each mpi_type
results = {"Problem Size": problem_sizes}
for mpi_type in mpi_types:
    results[mpi_type] = []

# Run the Rust programs and collect the results
for problem_size in problem_sizes:
    for mpi_type in mpi_types:
        command = f"mpirun -n 4 ./target/debug/fust {problem_size} {mpi_type}"
        try:
            # Run the command
            print(f"Now run: mpirun -n 4 ./target/debug/fust {problem_size} {mpi_type}")
            output = subprocess.check_output(command, shell=True, text=True)

            # Extract the time from the output
            time_str = output.split("time: ")[1].split(" ")[0]
            time_float = float(time_str)

            # Append the time to the results dictionary under the appropriate mpi_type
            results[mpi_type].append(time_float)
        except subprocess.CalledProcessError as e:
            print(f"Error running command '{command}': {e}")
            results[mpi_type].append(None)

# Create a DataFrame from the results and set the problem size as the index
df = pd.DataFrame(results)
df.set_index("Problem Size", inplace=True)

# Display the DataFrame
print(df)

# Plotting
plt.figure(figsize=(10, 6))
for mpi_type in mpi_types:
    plt.plot(df.index, df[mpi_type], marker='o', label=mpi_type)

plt.title('MPI Program Execution Time by Problem Size')
plt.xlabel('Problem Size')
plt.ylabel('Time (s)')
plt.legend()
plt.grid(True)
plt.xticks(df.index, rotation=45)
plt.tight_layout()

# save plot
plt.savefig("filename.png")

# Optionally, save the DataFrame to a CSV file
df.to_csv("mpi_rust_results_simplified.csv")
