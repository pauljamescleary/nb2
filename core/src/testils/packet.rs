use crate::packets::ip::v4::Ipv4;
use crate::packets::ip::v6::{Ipv6, SegmentRouting};
use crate::packets::{Ethernet, Packet, Tcp, Udp};

/// `Packet` extension trait.
///
/// Methods for packet conversion that make testing less verbose. Does not
/// guarantee that the result of the conversion will be a valid packet,
/// and will panic if the conversion fails.
pub trait PacketExt: Packet + Sized {
    fn into_eth(self) -> Ethernet {
        self.reset().parse::<Ethernet>().unwrap()
    }

    fn into_v4(self) -> Ipv4 {
        self.into_eth().parse::<Ipv4>().unwrap()
    }

    fn into_v4_tcp(self) -> Tcp<Ipv4> {
        self.into_v4().parse::<Tcp<Ipv4>>().unwrap()
    }

    fn into_v4_udp(self) -> Udp<Ipv4> {
        self.into_v4().parse::<Udp<Ipv4>>().unwrap()
    }

    fn into_v6(self) -> Ipv6 {
        self.into_eth().parse::<Ipv6>().unwrap()
    }

    fn into_v6_tcp(self) -> Tcp<Ipv6> {
        self.into_v6().parse::<Tcp<Ipv6>>().unwrap()
    }

    fn into_v6_udp(self) -> Udp<Ipv6> {
        self.into_v6().parse::<Udp<Ipv6>>().unwrap()
    }

    fn into_sr(self) -> SegmentRouting<Ipv6> {
        self.into_v6().parse::<SegmentRouting<Ipv6>>().unwrap()
    }

    fn into_sr_tcp(self) -> Tcp<SegmentRouting<Ipv6>> {
        self.into_sr().parse::<Tcp<SegmentRouting<Ipv6>>>().unwrap()
    }
}

impl<T> PacketExt for T where T: Packet + Sized {}
