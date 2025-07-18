use serde::{Deserialize, Serialize};
use crate::models::project::ProjectSearchResult;

/// Represents a CurseForge search request
#[derive(Debug, Clone, Serialize)]
pub struct SearchRequest {
    pub game_id: Option<u32>,
    pub class_id: Option<u32>,
    pub category_id: Option<u32>,
    pub game_version: Option<String>,
    pub search_filter: Option<String>,
    pub sort_field: Option<SortField>,
    pub sort_order: Option<SortOrder>,
    pub mod_loader_type: Option<ModLoaderType>,
    pub game_version_type_id: Option<u32>,
    pub author_id: Option<u32>,
    pub slug: Option<String>,
    pub index: Option<u32>,
    pub page_size: Option<u32>,
}

/// Represents sort field options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortField {
    Featured,
    Popularity,
    LastUpdated,
    Name,
    Author,
    TotalDownloads,
    Category,
    GameVersion,
    EarlyAccess,
    FeaturedDate,
    Relevance,
}

/// Represents sort order options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Represents mod loader types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ModLoaderType {
    Any,
    Forge,
    Cauldron,
    LiteLoader,
    Fabric,
    Quilt,
    NeoForge,
}

/// Represents a CurseForge search response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResponse {
    pub data: Vec<ProjectSearchResult>,
    pub pagination: Pagination,
}

/// Represents pagination information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pagination {
    pub index: u32,
    pub page_size: u32,
    pub result_count: u32,
    pub total_count: u32,
}

/// Represents a fingerprint search request
#[derive(Debug, Clone, Serialize)]
pub struct FingerprintSearchRequest {
    pub fingerprints: Vec<u64>,
}

/// Represents a fingerprint search response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FingerprintSearchResponse {
    pub data: Vec<FingerprintMatch>,
    pub exact_matches: Vec<u64>,
    pub partial_matches: Vec<u64>,
    pub unmatched_fingerprints: Vec<u64>,
}

/// Represents a fingerprint match
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FingerprintMatch {
    pub id: u32,
    pub file: FileMatch,
    pub latest_files: Vec<FileMatch>,
    pub fingerprints: Vec<u64>,
}

/// Represents a file match in fingerprint search
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileMatch {
    pub id: u32,
    pub display_name: String,
    pub file_name: String,
    pub file_date: String,
    pub file_length: u64,
    pub download_count: u64,
    pub download_url: Option<String>,
    pub game_versions: Vec<String>,
    pub sortable_game_versions: Vec<SortableGameVersion>,
    pub dependencies: Vec<FileDependency>,
    pub expose_as_alternative: Option<bool>,
    pub parent_project_file_id: Option<u32>,
    pub alternate_file_id: Option<u32>,
    pub is_available: bool,
    pub modules: Vec<FileModule>,
    pub package_fingerprint: u64,
    pub game_version_date_released: String,
    pub game_version_map: Vec<GameVersionMap>,
    pub install_metadata: Option<InstallMetadata>,
    pub changelog: Option<String>,
    pub has_install_script: bool,
    pub is_compatible_with_client: bool,
    pub category_section_package_type: u32,
    pub restrict_project_file_access: u32,
    pub project_status: u32,
    pub render_cache_id: Option<u32>,
    pub file_legacy_mapping_id: Option<u32>,
    pub project_id: u32,
    pub parent_project_id: Option<u32>,
    pub parent_file_legacy_mapping_id: Option<u32>,
    pub file_type_id: Option<u32>,
    pub package_fingerprint_id: u64,
    pub game_version_mapping_file_type: u32,
    pub game_version_mapping_type: u32,
    pub game_id: u32,
    pub is_server_pack: bool,
    pub server_pack_file_id: Option<u32>,
    pub game_display_name: String,
    pub sync: bool,
}

/// Represents a sortable game version
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SortableGameVersion {
    pub game_version_padded: String,
    pub game_version: String,
    pub game_version_release_date: String,
    pub game_version_name: String,
}

/// Represents a game version map
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameVersionMap {
    pub game_version_id: u32,
    pub game_version_name: String,
    pub game_version_type_id: Option<u32>,
    pub game_version_status: u32,
    pub game_version_type_status: u32,
}

/// Represents install metadata
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstallMetadata {
    pub folder_name: Option<String>,
    pub rename: Option<String>,
    pub file_id: u32,
}

/// Represents a file dependency
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileDependency {
    pub mod_id: u32,
    pub relation_type: RelationType,
}

/// Represents a file module
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileModule {
    pub name: String,
    pub fingerprint: u64,
}

/// Represents the relationship type between files
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationType {
    EmbeddedLibrary,
    OptionalDependency,
    RequiredDependency,
    Tool,
    Incompatible,
    Include,
} 