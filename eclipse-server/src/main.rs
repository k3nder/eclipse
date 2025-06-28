use config::Config;
use eclipse_protocol::Attachment;
use hmac::Mac;
use jwt::VerifyWithKey;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::ToSocketAddrs;
use tokio::task::spawn;
use tracing::*;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::SystemTime;
use tracing_subscriber::prelude::*;

use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_timer(SystemTime)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let settings = Config::builder()
        .add_source(config::File::with_name("eclipse.ini"))
        .add_source(config::Environment::with_prefix("ECL"))
        .build()?;

    let settings = Arc::new(settings);

    let listener = TcpListener::bind(
        settings
            .get_string("bind")
            .unwrap_or("0.0.0.0:7010".to_string()),
    )
    .await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("USER CONNECTED FROM {}", addr);
        let settings = settings.clone();
        spawn(async move {
            let settings = settings;
            let result = client(stream, addr, settings.clone()).await.unwrap();
        });
    }

    Ok(())
}
async fn client(
    mut stream: TcpStream,
    addr: SocketAddr,
    settings: Arc<config::Config>,
) -> anyhow::Result<()> {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).await.unwrap();
    let attachment_len = u32::from_be_bytes(len_bytes);
    if attachment_len > 1000 {
        return Err(anyhow::anyhow!("Attachment to long"));
    }
    let mut attachment = vec![0u8; attachment_len as usize];
    stream.read_exact(&mut attachment).await?;
    let attachment = Attachment::from_bytes(attachment)?;

    let claims: BTreeMap<String, String> =
        attachment.token.verify_with_key(&get_secret(settings)?)?;

    let user_name = claims.get("user").unwrap();
    info!("USER CONNECTED {}", user_name);

    let target = TcpStream::connect(attachment.ip).await?;
    server::redirect_stream(stream, target).await?;

    Ok(())
}
fn get_secret(settings: Arc<config::Config>) -> anyhow::Result<hmac::Hmac<sha2::Sha256>> {
    let secret_file = settings.get_string("secret")?;
    let str = std::fs::read_to_string(secret_file)?;
    Ok(hmac::Hmac::new_from_slice(str.as_bytes())?)
}
