use std::net::TcpStream;

use anyhow::Result;
use async_native_tls::{Certificate, TlsConnector};
use futures::io;
use futures::prelude::*;
use piper::Lock;
use smol::Async;

fn main() -> Result<()> {
    // Create a TLS connector that is able to connect to localhost:7001
    let mut builder = native_tls::TlsConnector::builder();
    builder.add_root_certificate(Certificate::from_pem(include_bytes!("../certificate.pem"))?);
    let tls = TlsConnector::from(builder);

    smol::run(async {
        let stdin = smol::reader(std::io::stdin());
        let mut stdout = smol::writer(std::io::stdout());

        let stream = Async::<TcpStream>::connect("localhost:7001").await?;
        let stream = tls.connect("localhost", stream).await?;

        println!("Connected to {}", stream.get_ref().get_ref().peer_addr()?);
        let stream = Lock::new(stream);

        future::try_join(
            io::copy(stdin, &mut &stream),
            io::copy(&stream, &mut stdout),
        )
        .await?;

        Ok(())
    })
}
