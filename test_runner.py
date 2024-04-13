import subprocess
import csv
import argparse

# Create the parser
parser = argparse.ArgumentParser()

# Add an argument
parser.add_argument('env', type=str, help="local or das6")
parser.add_argument('node', type=int, help="How many node?")
parser.add_argument('app', type=str, help="sor_source_data or ping pong")
parser.add_argument('round', type=int, help='# of iterations for SOR or # of rounds in pingpong')
parser.add_argument('--l2', type=int, nargs='+', help='matrix size for sor_source_data, vector size for pingpong')

# Parse the command line arguments
args = parser.parse_args()

# Define the problem sizes
round_num = args.round
problem_sizes = args.l2

# Initialize a list to hold the rows of the CSV file
csv_rows = []
proc_number = args.node

# Add the header row
header_row = ['Problem Size'] + ['sendrecv', 'rma']
csv_rows.append(header_row)

# Determine to run sor_source_data / pingpong
app = []
if args.app == 'sor_source_data':
    app = ['norm', 'rma']
elif args.app == 'pp':
    app = ['pin_norm', 'pin_rma']

# Run the Rust programs and collect the results
for problem_size in problem_sizes:
    current_row = [problem_size]  # Start the row with the problem size
    for de_app in app:
        command = ''
        if args.env == 'local':
            command = f"mpirun -n {proc_number} ./target/release/fust {problem_size} {round_num} {de_app}"
        elif args.env == 'das6':
            command = f"prun -np {proc_number} -1 -script $PRUN_ETC/prun-openmpi `pwd`/./target/release/fust {problem_size} {round_num} {de_app}"
        try:
            # Run the command
            print(f"Running {command} for 10 times")

            total_time_cost = 0
            for i in range(0, 10):
                output = subprocess.check_output(command, shell=True, universal_newlines=True)

                # Step 1: Split the string using the comma
                parts = output.split(',')

                # Step 2: Split the second part using the colon
                time_part = parts[1].split(':')

                # Step 3: Trim whitespace and remove the unit (if necessary)
                time_value = time_part[1].strip().split(' ')[0]

                total_time_cost += time_value

            # Append the averaged time to the current row
            current_row.append(total_time_cost / 10)
        except subprocess.CalledProcessError as e:
            print(f"Error running command '{command}': {e}")
            current_row.append('Error')  # Use 'Error' or None as appropriate

    # Add the current row to the list of rows for the CSV
    csv_rows.append(current_row)

# Write the results to a CSV file
with open(f'pp_node_{proc_number}.csv', 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerows(csv_rows)

print("CSV file has been generated.")
