use super::neighbor::Neighbor;
use std::collections::VecDeque;
use std::io::Read;
use std::net::{IpAddr, SocketAddr};

struct TcpNeighbor<T>
where
    T: Read,
{
    neighbor: Neighbor,
    send_queue: VecDeque<T>,
    stopped: bool,
    source: SocketAddr,
    sink: SocketAddr,
}
