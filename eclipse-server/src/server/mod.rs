use eclipse_protocol::Attachment;
use std::marker::Unpin;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;
use tokio::io::Error;
use tokio::io::copy;
use tokio::task::spawn;
use tokio::try_join;
use tracing::*;

pub async fn redirect<RA, WA, RB, WB>(
    mut ra: RA,
    mut wa: WA,
    mut rb: RB,
    mut wb: WB,
) -> Result<(u64, u64), std::io::Error>
where
    RA: AsyncRead + Send + Sync + Unpin,
    WA: AsyncWrite + Send + Sync + Unpin,
    RB: AsyncRead + Send + Sync + Unpin,
    WB: AsyncWrite + Send + Sync + Unpin,
{
    let read_to_write = copy(&mut ra, &mut wb);
    let write_to_read = copy(&mut rb, &mut wa);

    try_join!(read_to_write, write_to_read)
}
#[cfg(feature = "tcp")]
pub async fn redirect_stream(
    a: tokio::net::TcpStream,
    b: tokio::net::TcpStream,
) -> Result<(u64, u64), std::io::Error> {
    let (ra, wa) = a.into_split();
    let (rb, wb) = b.into_split();

    redirect(ra, wa, rb, wb).await
}
