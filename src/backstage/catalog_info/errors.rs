use crate::git::{FindGitRepoRootError, GetOriginRemoteRepoNameError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CatalogInfoExistsError {
    #[error("Failed to find the git repository root: {0}")]
    FailedToFindGitRepoRoot(#[from] FindGitRepoRootError),
}

#[derive(Error, Debug)]
pub enum CatalogInfoParseError {
    #[error("Failed to find the git repo root: {0}")]
    FailedToFindGitRepoRoot(#[from] FindGitRepoRootError),

    #[error("Couldn't read the catalog-info file: {0}")]
    FailedToReadCatalogInfoFile(#[from] std::io::Error),

    #[error("Could not parse the catalog-info file: {0}")]
    FailedToParseCatalogInfoFile(#[from] serde_yml::Error),
}

#[derive(Error, Debug)]
pub enum GetCatalogInfoError {
    #[error("Failed to check if a catalog-info file exists in the repository: {0}")]
    CheckCatalogInfoExists(#[from] CatalogInfoExistsError),

    #[error("Failed to parse repository catalog-info: {0}")]
    ParseCatalogInfo(#[from] CatalogInfoParseError),

    #[error("Failed to create new catalog-info file: {0}")]
    CreateNewCatalogInfo(#[from] NewCatalogInfoError),
}

#[derive(Error, Debug)]
pub enum NewCatalogInfoError {
    #[error("Failed to get the repo name due to error: {0}")]
    FailedToGetOriginRemoteRepoName(#[from] GetOriginRemoteRepoNameError),
}
