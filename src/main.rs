use anyhow::Result;
use std::vec::Vec;
use eigendatestharness::{EigenDA, EigenDAConfig, DAClient};

#[derive(clap::Parser)]
struct Cli {
    #[arg(long, global = true, default_value_t = 9148)]
    metrics_port: u16,
    #[arg(long, global = true)]
    stop: bool,
    #[command(subcommand)]
    cmd: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    EigenDADisperse(EigenDAConfig),
    EigenDAStore(EigenDAConfig),
}

async fn eigendadisperse(da: &EigenDA, data: &[u8]) {
    println!("dispersing blob");
    let _id = da.disperse_blob(&data).await.expect("request ids");
}

async fn eigendastore(da: &EigenDA, data: &[u8]) {
    println!("storing blob");
    let responses = da.store_blob(&data).await.expect("availability proofs");
    let data = da.retrieve_blob(responses).await.expect("retrieved data");
    for i in 0..data.len() {
        assert_eq!(data[i], i as u8);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("LayerN Rollman ");
    let Cli {
        metrics_port,
        stop,
        cmd,
    } = <Cli as clap::Parser>::parse();
    println!("{cmd:?}");

    let da_config = match &cmd {
        Command::EigenDAStore(cfg) => cfg.clone(),
        Command::EigenDADisperse(cfg) => cfg.clone(),
    };

    let addr = format!("0.0.0.0:{}", metrics_port);
    let metrics_server = tokio::task::spawn_blocking(move || {
        prometheus_exporter::start(addr.parse().expect("failed to parse binding"))
            .expect("failed to start prometheus exporter");
    });

    let mut data = Vec::<u8>::with_capacity(da_config.block_size);
    for i in 0..da_config.block_size {
        data.push(i as u8)
    }

    let da = EigenDA::new(da_config, prometheus::default_registry());

    loop {
        let start = std::time::Instant::now();
        match cmd {
            Command::EigenDAStore(_) => eigendastore(&da, &data).await,
            Command::EigenDADisperse(_) => eigendadisperse(&da, &data).await,
        };
        println!("Took {:?}", start.elapsed());
        if stop {
            break;
        }
    }

    let _ = metrics_server.await;
    Ok(())
}
