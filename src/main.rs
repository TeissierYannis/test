mod config;
mod whitelist;

use aya::{Bpf, programs::Xdp, XdpFlags};
use config::Config;
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use whitelist::Whitelist;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();
    env_logger::init();

    let whitelist = Arc::new(Mutex::new(Whitelist::new(config.htable_size)));

    if config.xdp {
        attach_xdp_program(&config).expect("Failed to load XDP program");
    }

    start_tcp_listener(config, whitelist.clone()).await?;

    Ok(())
}

fn attach_xdp_program(config: &Config) -> Result<(), aya::BpfError> {
    let mut bpf = Bpf::load_file("xdp.o")?;
    let program: &mut Xdp = bpf.program_mut("filter")?.try_into()?;
    program.load()?;
    program.attach("br0", XdpFlags::default())?;
    Ok(())
}

async fn start_tcp_listener(config: Config, whitelist: Arc<Mutex<Whitelist>>) -> std::io::Result<()> {
    let listener = TcpListener::bind((config.addr.as_str(), config.port)).await?;
    while let Ok((stream, _)) = listener.accept().await {
        let whitelist = whitelist.clone();
        tokio::spawn(async move {
            handle_client(stream, whitelist).await;
        });
    }
    Ok(())
}
