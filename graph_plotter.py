import pandas as pd
import matplotlib.pyplot as plt

proc_num = 4
# Load the data from the CSV file
df = pd.read_csv(f"results_node_{proc_num}.csv")

# Ensure 'Problem Size' is the x-axis
df.set_index("Problem Size", inplace=True)

# Plotting
plt.figure(figsize=(10, 6))

# Iterate through the columns (excluding 'Problem Size' since it's now the index) and plot
for column in df.columns:
    plt.plot(df.index, df[column], label=column, marker='o')

plt.title('MPI Performance Comparison')
plt.xlabel('Problem Size')
plt.ylabel('Time (ms)')
plt.legend()
plt.grid(True)

# Show the plot
plt.savefig(f"results_node_{proc_num}.png")
