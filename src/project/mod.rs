use crate::languages::Language;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Project {
    pub language: Box<dyn Language>,
    pub root: PathBuf,
}

impl Project {
    pub fn new(root: PathBuf, language: Box<dyn Language>) -> Project {
        Project { root, language }
    }

    pub fn get_internal_dependencies(&self) -> Vec<String> {
        let _internal_dependencies: Vec<String> = vec![];
        // internal_dependencies
        todo!();
    }
}
