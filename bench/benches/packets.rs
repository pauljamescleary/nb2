use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nb2::packets::ip::v4::Ipv4;
use nb2::packets::{Ethernet, Packet, Udp};
use nb2::testils::proptest::*;
use nb2::testils::{PacketExt, StrategyValGen};
use nb2::Mbuf;
use std::time::{Duration, Instant};

fn parse_udp_packet(mbuf: Mbuf) {
    let ethernet = mbuf.parse::<Ethernet>().unwrap();
    let ipv4 = ethernet.parse::<Ipv4>().unwrap();
    ipv4.parse::<Udp<Ipv4>>().unwrap();
}

fn deparse_udp_packet(udp: Udp<Ipv4>) {
    let d_ipv4 = udp.deparse();
    let d_eth = d_ipv4.deparse();
    d_eth.deparse();
}

#[nb2::bench]
fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("packets::parse_udp_packet", move |b| {
        b.iter_custom(|iters| {
            let mut gen = StrategyValGen::new();
            let mut total_elapsed = Duration::from_secs(0);
            for _i in 0..iters {
                let packet_mbuf = gen.generate(v4_udp());
                let start = Instant::now();
                black_box(parse_udp_packet(packet_mbuf));
                total_elapsed += start.elapsed()
            }
            total_elapsed
        })
    });

    c.bench_function("packets::deparse_udp_packet", move |b| {
        b.iter_custom(|iters| {
            let mut gen = StrategyValGen::new();
            let mut total_elapsed = Duration::from_secs(0);
            for _i in 0..iters {
                let packet = gen.generate(v4_udp()).into_v4_udp();
                let start = Instant::now();
                black_box(deparse_udp_packet(packet));
                total_elapsed += start.elapsed()
            }
            total_elapsed
        })
    });
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
