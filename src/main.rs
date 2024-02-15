mod use_mpi;
mod use_zeromq;
mod ping_pong_mpi;

fn main() {
    ping_pong_mpi::ping_pong();
}
