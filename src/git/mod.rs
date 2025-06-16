mod errors;
// use crate::project::Project;
use crate::languages::Languages;
use crate::project::Project;
use async_walkdir::WalkDir;
pub use errors::FindProjectsError;
pub use errors::{GetOriginRemoteRepoNameError, GitRepoTryFromError};
use futures_lite::stream::StreamExt;
use std::path::PathBuf;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct GitRepo {
    pub root: PathBuf,
    pub projects: Vec<Project>,
}

impl GitRepo {
    /// Finds the root of the Git repository by searching for the `.git` directory.
    pub async fn try_from(cwd: Option<PathBuf>) -> Result<GitRepo, GitRepoTryFromError> {
        let projects = vec![];
        let cwd = if let Some(path) = cwd {
            path
        } else {
            std::env::current_dir()
                .map_err(|source| GitRepoTryFromError::FailedToGetCWD { source })?
        };

        // check if .git directory exists in the current directory
        if cwd.join(".git").exists() {
            return Ok(GitRepo {
                root: cwd,
                projects,
            });
        }

        // if it does not then check up the directory tree until we hit the root
        let mut current_dir = cwd;
        loop {
            // check if we have reached the root directory
            if !current_dir.pop() {
                break; // we have reached the root directory
            }

            // check if .git directory exists in the current directory
            if current_dir.join(".git").exists() {
                return Ok(GitRepo {
                    root: current_dir,
                    projects,
                });
            }
        }

        // if not found then return an error stating we could not find the git repo root
        Err(GitRepoTryFromError::FailedToFindGitRepoRoot)
    }

    /// Get origin remote repo name
    pub async fn get_origin_remote_repo_name(
        &self,
    ) -> Result<String, GetOriginRemoteRepoNameError> {
        // read the origin remote from the git config
        let output = std::process::Command::new("git")
            .arg("-C")
            .arg(&self.root)
            .arg("remote")
            .arg("get-url")
            .arg("origin")
            .output()
            .map_err(|source| GetOriginRemoteRepoNameError::FailedToGetCWD { source })?;

        if !output.status.success() {
            return Err(GetOriginRemoteRepoNameError::FailedToFindGitRepoRoot(
                GitRepoTryFromError::FailedToFindGitRepoRoot,
            ));
        }

        // parse the output to get the repo name
        let url = String::from_utf8(output.stdout)
            .map_err(|_| GitRepoTryFromError::FailedToFindGitRepoRoot)?;
        let repo_name = url
            .trim()
            .split('/')
            .next_back()
            .unwrap_or_default()
            .to_string()
            .replace(".git", "");

        Ok(repo_name)
    }

    /// Find projects
    pub async fn find_projects(&mut self) -> Result<(), FindProjectsError> {
        let mut entries = WalkDir::new(&self.root);

        // iterate over all directories in the git repo root
        loop {
            match entries.next().await {
                Some(Ok(entry)) => {
                    for language in Languages::iter() {
                        if entry.path().is_dir() {
                            continue;
                        }

                        let path_string = entry
                            .path()
                            .file_name()
                            .ok_or_else(|| FindProjectsError::FailedToGetFileName {
                                path: format!("{}", entry.path().display()),
                            })?
                            .to_str()
                            .ok_or_else(|| FindProjectsError::FailedToGetFileName {
                                path: format!("{}", entry.path().display()),
                            })?
                            .to_string();

                        if path_string == language.get_project_file_name() {
                            self.projects.push(Project::new(
                                entry
                                    .path()
                                    .parent()
                                    .ok_or_else(|| FindProjectsError::FailedToGetProjectRootPath {
                                        path: format!("{}", entry.path().display()),
                                    })?
                                    .to_path_buf(),
                                language.get_language(),
                            ));
                        }
                    }
                }
                Some(Err(err)) => {
                    eprintln!("error: {err}");
                    break;
                }
                None => break,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_try_from() -> Result<()> {
        // make a temporary directory
        let test_dir = tempdir()?;

        // create a .git directory within it
        let git_dir_path = test_dir.path().join(".git");
        std::fs::create_dir(&git_dir_path)?;

        // This test will only pass if the current working directory is a git repository
        let result = GitRepo::try_from(Some(test_dir.path().into())).await;

        assert!(result.is_ok(), "Failed to find git repo root: {result:?}");

        // check the directory returned is the test_dir
        assert_eq!(test_dir.path(), result?.root);

        Ok(())
    }

    #[tokio::test]
    async fn test_find_git_repo_root_not_found() -> Result<()> {
        // This test will fail if the current working directory is a git repository
        let test_dir = tempdir()?;
        let result = GitRepo::try_from(Some(test_dir.path().into())).await;
        assert!(result.is_err(), "Expected error but found: {result:?}");
        Ok(())
    }

    #[tokio::test]
    async fn test_find_git_repo_root_recursive() -> Result<()> {
        // make a temporary directory
        let test_dir = tempdir()?;

        // create a .git directory within it
        let git_dir_path = test_dir.path().join(".git");
        std::fs::create_dir(&git_dir_path)?;

        // create another directory inside the test directory
        let nested_dir = test_dir.path().join("nested");

        // This test will only pass if the current working directory is a git repository
        let result = GitRepo::try_from(Some(nested_dir)).await;

        assert!(
            result.is_ok(),
            "Failed to find git repo root during recursive test: {result:?}"
        );

        // confirm the returned directory is the test_dir
        assert_eq!(test_dir.path(), result?.root);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_origin_remote_repo_name() -> Result<()> {
        // make a temporary directory
        let test_dir = tempdir()?;

        // init a git repo inside the test dir
        std::process::Command::new("git")
            .arg("-C")
            .arg(test_dir.path())
            .arg("init")
            .output()?;

        // create a fake origin remote
        std::process::Command::new("git")
            .arg("-C")
            .arg(test_dir.path())
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg("git@bitbucket.org:acompany/myrepo.git")
            .output()?;

        // get the origin remote
        let remote = GitRepo::try_from(Some(test_dir.path().into()))
            .await?
            .get_origin_remote_repo_name()
            .await?;

        // verify the repo name matches
        assert_eq!(remote, "myrepo");

        Ok(())
    }
}
