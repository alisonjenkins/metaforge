mod errors;

use crate::git::{find_git_repo_root, get_origin_remote_repo_name};
pub use errors::{
    CatalogInfoExistsError, CatalogInfoParseError, GetCatalogInfoError, NewCatalogInfoError,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const CATALOG_INFO_FILE: &str = "catalog-info.yaml";

/// A struct that describes a Backstage catalog info file.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CatalogInfo {
    /// The api version of the backstage catalog info file.
    #[serde(rename = "apiVersion")]
    pub api_version: String,

    /// The backstage catalog-info kind
    pub kind: String,

    /// The metadata for the catalog-info file
    pub metadata: CatalogInfoMetadata,

    /// The catalog-info spec
    pub spec: CatalogInfoSpec,
}

/// A struct that represents the metadata of a Backstage catalog info file.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CatalogInfoMetadata {
    /// The name of the entity in the catalog
    pub name: String,

    /// The description of the entity in the catalog
    pub description: String,

    /// The annotations of the entity
    pub annotations: BTreeMap<String, String>,

    /// The tags of the entity
    pub tags: BTreeMap<String, String>,

    /// The links for the entity
    pub links: Vec<CatalogInfoMetadataLink>,
}

/// A struct that represents a link in the Backstage catalog info metadata.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CatalogInfoMetadataLink {
    /// An optional icon for the link
    pub icon: Option<String>,

    /// The title of the link
    pub title: String,

    /// The url of the link
    pub url: String,
}

/// A struct that represents the spec of a Backstage catalog info file.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CatalogInfoSpec {
    /// The lifecycle of the entity
    pub lifecycle: String,

    /// The owner of the entity
    pub owner: String,

    /// The type of the entity
    pub r#type: String,

    /// The system that the entity belongs to
    pub system: Option<String>,

    /// The dependencies of the entity
    #[serde(rename = "dependsOn")]
    pub depends_on: Vec<String>,
}

impl CatalogInfo {
    /// If a catalog-info file exists in the current repository it parses it and returns it's data.
    /// Otherwise creates a new Backstage Catalog info file and returns it's initial data.
    pub async fn get() -> Result<CatalogInfo, GetCatalogInfoError> {
        // check if the catalog info file exists
        if CatalogInfo::exists().await? {
            // if it does then parse it and return the data
            Ok(CatalogInfo::parse().await?)
        } else {
            // if it does not then create a new one and return the initial data
            Ok(CatalogInfo::new().await?)
        }
    }

    /// Creates a new Backstage Catalog info file and returns it's initial data.
    pub async fn new() -> Result<CatalogInfo, NewCatalogInfoError> {
        let repo_name = get_origin_remote_repo_name(None).await?;

        Ok(CatalogInfo {
            api_version: "backstage.io/v1alpha1".to_string(),
            kind: "Component".to_string(),
            metadata: CatalogInfoMetadata {
                name: repo_name.clone(),
                description: format!(
                    "A Backstage catalog info file for the {} repository",
                    repo_name
                ),
                annotations: BTreeMap::new(),
                tags: BTreeMap::new(),
                links: vec![],
            },
            spec: CatalogInfoSpec {
                lifecycle: "experimental".to_string(),
                owner: "test".to_string(),
                r#type: "service".to_string(),
                system: Some("a_system".to_string()),
                depends_on: vec![],
            },
        })
    }

    /// Checks if the Backstage catalog info file exists in the repository root.
    pub async fn exists() -> Result<bool, CatalogInfoExistsError> {
        // find the root of the repository
        let repo_root = find_git_repo_root(None).await?;

        // check the file exists
        Ok(repo_root.join(CATALOG_INFO_FILE).exists())
    }

    /// Parses the Backstage Catalog info file if it exists
    pub async fn parse() -> Result<CatalogInfo, CatalogInfoParseError> {
        let catalog_info_file =
            std::fs::File::open(find_git_repo_root(None).await?.join(CATALOG_INFO_FILE))?;
        Ok(serde_yml::from_reader(catalog_info_file)?)
    }
}
