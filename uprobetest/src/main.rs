use aya::programs::UProbe;
use aya::{include_bytes_aligned, Bpf};
use aya_log::BpfLogger;
use clap::Parser;
use log::{info, warn};
use std::env;
use tokio::signal;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long)]
    pid: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    env_logger::init();

    let file_path = "/home/afredette/projects/uprobetest/bpf/uretprobe.bpf.o";

    let mut bpf = Bpf::load_file(file_path)?;

    // if let Err(e) = BpfLogger::init(&mut bpf) {
    //     // This can happen if you remove all log statements from your eBPF program.
    //     warn!("failed to initialize eBPF logger: {}", e);
    // }

    let program: &mut UProbe = bpf.program_mut("probe").unwrap().try_into()?;
    program.load()?;

    info!("Program Kind: {:?}", program.kind());

    // let target = env::current_dir().unwrap().join("samples/hello/hello");

    // let target = "/usr/bin/ping";

    //program.attach(Some("main"), 0, target, opt.pid)?;
    program.attach(Some("malloc"), 4, "libc", opt.pid)?;

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");

    Ok(())
}
