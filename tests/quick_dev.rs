use anyhow::Result;

#[tokio::test]

async fn quick_dev() -> Result<()>{

    let hc = httpc_tests::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?;

    Ok(())
}