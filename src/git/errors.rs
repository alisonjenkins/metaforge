use thiserror::Error;

#[derive(Error, Debug)]
pub enum FindGitRepoRootError {
    #[error("Could not get current working directory: {source}")]
    FailedToGetCWD { source: std::io::Error },

    #[error(
        "Failed to find the Git repository root. Have you initialized a Git repository or are you running the tool in the wrong directory?"
    )]
    FailedToFindGitRepoRoot,
}

#[derive(Error, Debug)]
pub enum GetOriginRemoteRepoNameError {
    #[error("Could not get current working directory: {source}")]
    FailedToGetCWD { source: std::io::Error },

    #[error("Failed to find git repository root: {0}")]
    FailedToFindGitRepoRoot(#[from] FindGitRepoRootError),
}
