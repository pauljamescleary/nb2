use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nb2::packets::ip::v4::Ipv4;
use nb2::packets::{Ethernet, Packet, Udp};
use nb2::testils::proptest::*;
use nb2::testils::{PacketExt, StrategyValGen};
use nb2::Mbuf;
use proptest::collection::vec;
use proptest::prelude::*;
use std::ops::RangeInclusive;
use std::time::{Duration, Instant};

const BATCH_SIZE: usize = 500;
const BATCH_RANGE: RangeInclusive<usize> = BATCH_SIZE..=BATCH_SIZE + 1;

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

#[nb2::bench(mempool_capacity = 512)]
fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("packets::parse_udp_packets", move |b| {
        b.iter_custom(|iters| {
            let mut total_elapsed = Duration::from_secs(0);
            for _i in 0..iters {
                let mut gen = StrategyValGen::new();
                let udp_mbufs = gen.generate(vec(v4_udp(), BATCH_RANGE));
                let start = Instant::now();
                for mbuf in udp_mbufs {
                    black_box(parse_udp_packet(mbuf))
                }
                total_elapsed += start.elapsed()
            }
            total_elapsed
        })
    });

    c.bench_function("packets::deparse_udp_packets", move |b| {
        b.iter_custom(|iters| {
            let mut total_elapsed = Duration::from_secs(0);
            for _i in 0..iters {
                let mut gen = StrategyValGen::new();
                let udps = gen.generate(vec(v4_udp().prop_map(|v| v.into_v4_udp()), BATCH_RANGE));
                let start = Instant::now();
                for udp in udps {
                    black_box(deparse_udp_packet(udp))
                }
                total_elapsed += start.elapsed()
            }
            total_elapsed
        })
    });
}

fn bench_config() -> Criterion {
    Criterion::default().sample_size(50)
}

criterion_group! {
    name = benches;
    config=bench_config();
    targets=parse_benchmark
}

criterion_main!(benches);
