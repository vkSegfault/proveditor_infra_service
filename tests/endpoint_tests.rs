use anyhow::{Result, Ok};

#[tokio::test]
async fn get_infra() -> Result<()> {
    
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/api/v1/infra").await?.print().await?;

    Ok(())
}