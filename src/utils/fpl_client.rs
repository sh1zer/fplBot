use anyhow::Result;
use fpl_client::client::FplApiClient;
use tokio::sync::OnceCell;

static FPL_CLIENT: OnceCell<FplApiClient> = OnceCell::const_new();

pub fn init_fpl_client() -> Result<()> {
    let client = FplApiClient::new()?;
    FPL_CLIENT
        .set(client)
        .map_err(|_| anyhow::anyhow!("Client already initialized"))?;

    Ok(())
}

pub fn fpl_client() -> &'static FplApiClient {
    FPL_CLIENT.get().expect("Fpl client not initialized")
}

