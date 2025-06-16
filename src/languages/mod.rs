pub mod errors;
pub mod go;
pub mod rust;
// use crate::backstage::Component;
// use std::error::Error;
use crate::backstage::Component;
use errors::GetInternalDependenciesError;
use std::fmt::{Debug, Display};
use std::path::Path;
use strum_macros::EnumIter;

pub trait Language: Dependencies + Display + Debug {}

pub trait Dependencies {
    fn get_internal_dependencies(
        &self,
        project_root: &Path,
    ) -> Result<Vec<Component>, GetInternalDependenciesError>;
    fn get_project_file_name(&self) -> String;
}

#[derive(Debug, EnumIter)]
pub enum Languages {
    Go(go::Go),
    Rust(rust::Rust),
}

impl Languages {
    pub fn get_project_file_name(&self) -> String {
        match self {
            Languages::Go(lang) => lang.get_project_file_name(),
            Languages::Rust(lang) => lang.get_project_file_name(),
        }
    }

    pub fn get_language(&self) -> Box<dyn Language> {
        match self {
            Languages::Go(lang) => Box::new(lang.clone()),
            Languages::Rust(lang) => Box::new(lang.clone()),
        }
    }
}
