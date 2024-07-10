#### Compile & Run locally
```
cargo build
mpirun -n <proc num> ./target/debug/pingpong {size}
mpicc -O3 src/ping_pong_rma.c -o c_rma -lm
```

#### Run MPI code in DAS6
```
prun -np 2 -1 -script $PRUN_ETC/prun-openmpi `pwd`/./c_rma 31
prun -np 2 -1 OMPI_OPTS="--mca btl tcp,self --mca btl_tcp_if_include eth4" -script $PRUN_ETC/prun-openmpi `pwd`/./c_rma 31
prun -np 2 -1 -script $PRUN_ETC/prun-openmpi `pwd`/./target/release/pingpong raw 31
prun -np 2 -1 OMPI_OPTS="--mca btl tcp,self --mca btl_tcp_if_include ib0" -script $PRUN_ETC/prun-openmpi `pwd`/./target/release/pingpong rma 32
prun -np 2 -1 OMPI_OPTS="--mca btl tcp,self --mca btl_tcp_if_include eth4" -script $PRUN_ETC/prun-openmpi `pwd`/./target/release/pingpong rma 32
```
