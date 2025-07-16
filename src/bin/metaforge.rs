use anyhow::Result;
use clap::Parser;
use metaforge::backstage::CatalogInfo;
use metaforge::cli::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let _args = Args::parse();

    let catalog_info = CatalogInfo::get().await?;

    print!("{catalog_info:?}");

    // scan the repo for a go.mod file
    // find the internal dependencies
    Ok(())
}
