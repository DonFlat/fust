#### Compile & Run locally
```
cargo build
mpirun -n <proc num> ./target/debug/pingpong {size}
```

#### Run MPI code in DAS6
```
prun -np <node num> -<proc num> -script $PRUN_ETC/prun-openmpi `pwd`/./target/debug/fust 
prun -np 2 -1 OMPI_OPTS="--mca btl tcp,self --mca btl_tcp_if_include ib0" -script $PRUN_ETC/prun-openmpi `pwd`/./target/release/pingpong rma 32
```
