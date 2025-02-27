//!
//! # Delete Managed Connectors
//!
//! CLI tree to generate Delete Managed Connectors
//!
use structopt::StructOpt;

use fluvio::Fluvio;
use fluvio::metadata::connector::ManagedConnectorSpec;

use crate::CliError;

// -----------------------------------
// CLI Options
// -----------------------------------

#[derive(Debug, StructOpt)]
pub struct DeleteManagedConnectorOpt {
    /// The name of the connector to delete
    #[structopt(value_name = "name")]
    name: String,
}

impl DeleteManagedConnectorOpt {
    pub async fn process(self, fluvio: &Fluvio) -> Result<(), CliError> {
        let admin = fluvio.admin().await;
        admin.delete::<ManagedConnectorSpec, _>(&self.name).await?;
        Ok(())
    }
}
