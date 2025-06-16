use thiserror::Error;

#[derive(Error, Debug)]
pub enum GetInternalDependenciesError {
    #[error("Failed to compile require block regex: {source}")]
    FailedToCompileRequireBlockRegex { source: regex::Error },

    #[error("Failed to compile require line regex: {source}")]
    FailedToCompileRequireLineRegex { source: regex::Error },

    #[error("Failed to compile internal repo regex: {source}")]
    FailedToCompileInternalRepoRegex { source: regex::Error },

    #[error("Failed to read the project Dependencies file: {source}")]
    FailedToReadProjectDependenciesFile { source: std::io::Error },
}
