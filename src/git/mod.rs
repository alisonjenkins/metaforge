mod errors;
pub use errors::{FindGitRepoRootError, GetOriginRemoteRepoNameError};
use std::path::PathBuf;

/// Find the root of the Git repository by searching for the `.git` directory.
pub async fn find_git_repo_root(cwd: Option<PathBuf>) -> Result<PathBuf, FindGitRepoRootError> {
    let cwd = if let Some(path) = cwd {
        path
    } else {
        std::env::current_dir().map_err(|source| FindGitRepoRootError::FailedToGetCWD { source })?
    };

    // check if .git directory exists in the current directory
    if cwd.join(".git").exists() {
        return Ok(cwd);
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
            return Ok(current_dir);
        }
    }

    // if not found then return an error stating we could not find the git repo root
    Err(FindGitRepoRootError::FailedToFindGitRepoRoot)
}

/// Get origin remote repo name
pub async fn get_origin_remote_repo_name(
    cwd: Option<PathBuf>,
) -> Result<String, GetOriginRemoteRepoNameError> {
    let cwd = if let Some(path) = cwd {
        path
    } else {
        std::env::current_dir().map_err(|source| FindGitRepoRootError::FailedToGetCWD { source })?
    };

    // find the root of the repository
    let repo_root = find_git_repo_root(Some(cwd)).await?;

    // read the origin remote from the git config
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .output()
        .map_err(|source| GetOriginRemoteRepoNameError::FailedToGetCWD { source })?;

    if !output.status.success() {
        return Err(GetOriginRemoteRepoNameError::FailedToFindGitRepoRoot(
            FindGitRepoRootError::FailedToFindGitRepoRoot,
        ));
    }

    // parse the output to get the repo name
    let url = String::from_utf8(output.stdout)
        .map_err(|_| FindGitRepoRootError::FailedToFindGitRepoRoot)?;
    let repo_name = url
        .trim()
        .split('/')
        .next_back()
        .unwrap_or_default()
        .to_string()
        .replace(".git", "");

    Ok(repo_name)
}

// create test for find_git_repo_root
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_find_git_repo_root() -> Result<()> {
        // make a temporary directory
        let test_dir = tempdir()?;

        // create a .git directory within it
        let git_dir_path = test_dir.path().join(".git");
        std::fs::create_dir(&git_dir_path)?;

        // This test will only pass if the current working directory is a git repository
        let result = find_git_repo_root(Some(test_dir.path().into())).await;

        assert!(result.is_ok(), "Failed to find git repo root: {:?}", result);

        // check the directory returned is the test_dir
        assert_eq!(test_dir.path(), result?);

        Ok(())
    }

    #[tokio::test]
    async fn test_find_git_repo_root_not_found() -> Result<()> {
        // This test will fail if the current working directory is a git repository
        let test_dir = tempdir()?;
        let result = find_git_repo_root(Some(test_dir.path().into())).await;
        assert!(result.is_err(), "Expected error but found: {:?}", result);
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
        let result = find_git_repo_root(Some(nested_dir)).await;

        assert!(
            result.is_ok(),
            "Failed to find git repo root during recursive test: {:?}",
            result
        );

        // confirm the returned directory is the test_dir
        assert_eq!(test_dir.path(), result?);

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
        let remote = get_origin_remote_repo_name(Some(test_dir.path().into())).await?;

        // verify the repo name matches
        assert_eq!(remote, "myrepo");

        Ok(())
    }
}
