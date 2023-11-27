use backon::{ConstantBuilder, BackoffBuilder, BlockingRetryable, ExponentialBuilder, ConstantBackoff, Retryable};
use std::time::Duration;
use anyhow::{Result, bail};

fn failing_network_operation() -> Result<i32> {
    bail!("error")
}

async fn failing_network_operation2() -> Result<i32> {
    bail!("error")
}

#[tokio::main]
async fn main() {
    async_main().await;
}

fn sync_main() {
    let constant = ConstantBuilder::default();
    let constant = constant.with_delay(Duration::from_secs(5)).with_max_times(5);

    let foo = failing_network_operation.retry(&constant).call();
    println!("foo: {foo:?}");

    println!("Hello, world!");
}

async fn async_main() -> Result<()> {
    let constant = ConstantBuilder::default();
    let constant = constant.with_delay(Duration::from_secs(5)).with_max_times(5);

    let foo = failing_network_operation2.retry(&constant).await;
    println!("foo: {foo:?}");

    println!("Hello, world!");
    Ok(())
}
