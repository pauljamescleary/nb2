//! Implementations of `PacketRx` and `PacketTx`.
//!
//! Implemented for `PortQueue`.
//!
//! `PacketRx` implemented for `KniRx`.
//!
//! `PacketTx` implemented for `KniTxQueue`.
//!
//! Implemented for `Vec` so it can be used as the batch source mostly
//! in tests.

use super::{PacketRx, PacketTx};
use crate::{KniRx, KniTxQueue, Mbuf, PortQueue};

impl PacketRx for PortQueue {
    fn receive(&mut self) -> Vec<Mbuf> {
        PortQueue::receive(self)
    }
}

impl PacketTx for PortQueue {
    fn transmit(&mut self, packets: Vec<Mbuf>) {
        PortQueue::transmit(self, packets)
    }
}

impl PacketRx for KniRx {
    fn receive(&mut self) -> Vec<Mbuf> {
        KniRx::receive(self)
    }
}

impl PacketTx for KniTxQueue {
    fn transmit(&mut self, packets: Vec<Mbuf>) {
        KniTxQueue::transmit(self, packets)
    }
}

impl PacketRx for Vec<Mbuf> {
    fn receive(&mut self) -> Vec<Mbuf> {
        self.drain(..).collect()
    }
}

impl PacketTx for Vec<Mbuf> {
    fn transmit(&mut self, packets: Vec<Mbuf>) {
        self.extend_from_slice(&packets)
    }
}

pub struct PollRx<F>
where
    F: Fn() -> Vec<Mbuf>,
{
    pub(crate) f: F,
}

impl<F> PacketRx for PollRx<F>
where
    F: Fn() -> Vec<Mbuf>,
{
    fn receive(&mut self) -> Vec<Mbuf> {
        (self.f)()
    }
}
