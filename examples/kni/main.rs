use nb2::packets::{Ethernet, Packet};
use nb2::settings::load_config;
use nb2::{Batch, KniRx, Pipeline, Poll, PortQueue, Result, Runtime};
use tracing::{debug, Level};
use tracing_subscriber::fmt;

fn install_rx(q: PortQueue) -> impl Pipeline {
    Poll::new(q.clone())
        .for_each(|packet| {
            let eth = packet.peek::<Ethernet>()?;
            println!("to kni: {:?}", eth);
            Ok(())
        })
        .send(q.kni().unwrap().clone())
}

fn install_tx(kni: KniRx, q: PortQueue) -> impl Pipeline {
    Poll::new(kni)
        .for_each(|packet| {
            let eth = packet.peek::<Ethernet>()?;
            println!("from kni: {:?}", eth);
            Ok(())
        })
        .send(q)
}

fn main() -> Result<()> {
    let subscriber = fmt::Subscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let config = load_config()?;
    debug!(?config);

    Runtime::build(config)?
        .add_pipeline_to_port("kni0", install_rx)?
        .add_kni_rx_pipeline_to_port("kni0", install_tx)?
        .execute()
}
