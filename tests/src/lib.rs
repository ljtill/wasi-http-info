mod helper;

use crate::helper::*;
use anyhow::Result;

#[tokio::test]
async fn it_returns_version() -> Result<()> {
    // TODO: Wire in HTTP Port

    let mut child = helper::wasmtime_serve(Some(&format!(
        "target/wasm32-wasi/debug/wasi_http_info.wasm"
    )))?;

    let response = helper::new_request("/version".to_string()).await?;
    let body = parse_response_body(response).await?;

    assert_eq!(body, "{\"commit\":\"6298f3d\"}");

    child.kill().expect("failed to kill process");
    Ok(())
}

#[tokio::test]
async fn it_returns_environment() -> Result<()> {
    // TODO: Wire in HTTP Port

    let mut child = wasmtime_serve(Some(&format!(
        "target/wasm32-wasi/debug/wasi_http_info.wasm"
    )))?;

    let response = new_request("/environment".to_string()).await?;
    let body = parse_response_body(response).await?;

    assert_eq!(body, "{}");

    child.kill().expect("failed to kill process");
    Ok(())
}
