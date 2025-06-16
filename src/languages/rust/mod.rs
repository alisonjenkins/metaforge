use super::errors::GetInternalDependenciesError;
use crate::backstage::Component;
use crate::languages::Dependencies;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct Rust {}

impl Dependencies for Rust {
    fn get_project_file_name(&self) -> String {
        "Cargo.toml".to_string()
    }

    fn get_internal_dependencies(
        &self,
        _project_root: &Path,
    ) -> Result<Vec<Component>, GetInternalDependenciesError> {
        let components = vec![];
        let _internal_repo_regex = r#".*bitbucket.org[\/:]bxbdigital\/.*"#;

        Ok(components)
    }
}

impl crate::languages::Language for Rust {}

impl std::fmt::Display for Rust {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Rust")
    }
}
