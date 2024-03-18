mod use_mpi;
mod use_zeromq;
mod ping_pong_mpi;
mod sor_seq;
mod sor;
mod sor_rma;

fn main() {
    sor::sor();
}
