use anyhow::Result;
use clap::Parser;
// use metaforge::backstage::CatalogInfo;
use metaforge::cli::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let _args = Args::parse();

    let mut repo = metaforge::git::GitRepo::try_from(None).await?;
    repo.find_projects().await?;

    for project in &repo.projects {
        // project.root
    }

    println!("{repo:?}");

    Ok(())
}
