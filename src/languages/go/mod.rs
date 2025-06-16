use super::errors::GetInternalDependenciesError;
use crate::backstage::Component;
use crate::languages::{Dependencies, Language};
use regex::Regex;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct Go {}

impl Dependencies for Go {
    fn get_project_file_name(&self) -> String {
        "go.mod".to_string()
    }

    fn get_internal_dependencies(
        &self,
        project_root: &Path,
    ) -> Result<Vec<Component>, GetInternalDependenciesError> {
        let require_block_regex = Regex::new(r#"(require \((?:.*\n)*?\))"#).map_err(|source| {
            GetInternalDependenciesError::FailedToCompileRequireBlockRegex { source }
        })?;
        let require_line_regex = Regex::new(r#"((\t(.+?) (.+)\n))"#).map_err(|source| {
            GetInternalDependenciesError::FailedToCompileRequireLineRegex { source }
        })?;
        let internal_repo_regex =
            Regex::new(r#".*bitbucket.org[\/:]bxbdigital\/.*"#).map_err(|source| {
                GetInternalDependenciesError::FailedToCompileInternalRepoRegex { source }
            })?;

        let mut components = vec![];

        let project_file_text = std::fs::read_to_string(
            project_root.join(self.get_project_file_name()),
        )
        .map_err(|source| {
            GetInternalDependenciesError::FailedToReadProjectDependenciesFile { source }
        })?;

        for require_block in require_block_regex.captures_iter(&project_file_text) {
            let require_block = require_block.get(1).map_or("", |m| m.as_str());
            for require_line in require_line_regex.captures_iter(require_block) {
                let module_name = require_line.get(3).map_or("", |m| m.as_str());

                if internal_repo_regex.is_match(module_name) {
                    components.push(Component {
                        name: module_name.to_string(),
                    });
                }
            }
        }

        Ok(components)
    }
}

impl Language for Go {}

impl std::fmt::Display for Go {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Go")
    }
}
