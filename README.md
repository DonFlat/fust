#### Compile & Run locally
```
cargo build
mpirun -n <proc num> ./target/debug/fust {size} {iteration} {program}
```

#### Run MPI code in DAS6
```
prun -np <node num> -<proc num> -script $PRUN_ETC/prun-openmpi `pwd`/./target/debug/fust 
```
