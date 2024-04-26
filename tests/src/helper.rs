use anyhow::{Error, Ok, Result};
use http_body_util::{BodyExt, Empty};
use hyper::{
    body::{Bytes, Incoming},
    client::conn::http1,
    Request, Response,
};
use hyper_util::rt::TokioIo;
use std::{
    env::consts,
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};
use tokio::net::TcpStream;

pub async fn new_request(path: String) -> Result<Response<Incoming>> {
    let localhost = format!("http://localhost:8080{}", path);
    let url = localhost.parse::<hyper::Uri>()?;
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let address = format!("{}:{}", host, port);

    let stream = TcpStream::connect(address).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = http1::handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    Ok(sender.send_request(req).await?)
}

pub async fn parse_response_body(mut response: Response<Incoming>) -> Result<String> {
    let mut response_string = String::new();
    while let Some(next) = response.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            response_string.push_str(std::str::from_utf8(chunk)?);
        }
    }

    Ok(response_string)
}

fn check_host() -> Result<()> {
    if consts::OS == "linux" {
        Ok(())
    } else {
        Err(Error::msg("unsupported OS"))
    }
}

pub fn wasmtime_serve(args: Option<&str>) -> Result<Child> {
    check_host()?;

    let mut cmd = Command::new("/usr/local/bin/wasmtime");
    cmd.arg("serve");
    cmd.stderr(Stdio::null());
    cmd.stdout(Stdio::null());

    if let Some(args) = args {
        for arg in args.split_whitespace() {
            cmd.arg(arg);
        }
    }

    let mut child = cmd.spawn().expect("failed to execute process");
    thread::sleep(Duration::from_secs(1));

    match child.try_wait()? {
        Some(_status) => Err(Error::msg("process terminated unexpectedly")),
        None => Ok(child),
    }
}
