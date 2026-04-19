pub mod cmd;
pub mod internal;
pub mod prelude;

use crate::prelude::Result;

#[tokio::main]
async fn main() -> Result<()>{
    cmd::run().await?;
    Ok(())
}
