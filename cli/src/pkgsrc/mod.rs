//! `pkgsrc` (Package Source) module provides functions to fetch package files from the internet.
//! It is used when a package cannot be installed through system package manager,
//! or when system package manager is unavailable on certain platforms.
//!
//! Designed with a failover mechanism, it allows for multiple sources to be used.
//! If one source fails, the next one takes over.
//!
//! The maintainers of STM32Tesseract have created a primary source
//! to host all the necessary packages, which is the preferred source.
//! Additionally, it supports fetching packages from upstream sources (e.g., GitHub Releases),
//! which serves as a fallback in case of failure.

mod campanula;
mod github_release;
mod pkg_index;
use pkg_index::PackageIndex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FetchPackageError {
    #[error("Package not found")]
    PackageNotFound,
    #[error("Failed to fetch package from all sources: {0:?}")]
    FetchFailed(Vec<(String, GenericError)>),
}

#[derive(Debug, Error)]
pub enum MatchAssetError {
    #[error("Asset not found")]
    AssetNotFound,
}

pub trait PackageAsset {
    fn name(&self) -> &str;
    // This may consume a lot of time, as it may involve verification.
    fn download_url(&self) -> &str;
}
type GenericPackageAsset = Box<dyn PackageAsset + Send + Sync + 'static>;

pub struct PackageInfo {
    /// package name
    name: String,
    /// eg. tag in git system
    version_name: String,
    /// eg. commit hash in git system
    version_code: String,
    assets: Vec<GenericPackageAsset>,
}

impl PackageInfo {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[allow(dead_code)]
    pub fn version_name(&self) -> &str {
        &self.version_name
    }
    #[allow(dead_code)]
    pub fn version_code(&self) -> &str {
        &self.version_code
    }
    #[allow(dead_code)]
    pub fn assets(&self) -> &[GenericPackageAsset] {
        &self.assets
    }
    pub fn match_asset<P>(&self, mut predicate: P) -> Result<&GenericPackageAsset, MatchAssetError>
    where
        P: FnMut(&GenericPackageAsset) -> bool,
    {
        for asset in &self.assets {
            if predicate(asset) {
                return Ok(asset);
            }
        }
        Err(MatchAssetError::AssetNotFound)
    }
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

trait PackageAcquirer {
    fn latest(&self) -> GenericResult<PackageInfo>;
    fn source_name(&self) -> String;
}

type GenericPackageAcquirer = Box<dyn PackageAcquirer + Send + Sync + 'static>;

static PKG_INDEX: std::sync::OnceLock<PackageIndex> = std::sync::OnceLock::new();

fn get_index() -> &'static PackageIndex {
    PKG_INDEX.get_or_init(|| {
        let mut index =
            campanula::index_of_campanula("https://stm32tesseract.alampy.com/campanula");
        index.merge(github_release::index_of_github_release());
        index
    })
}

pub fn fetch_package(name: &str) -> Result<PackageInfo, FetchPackageError> {
    let index = get_index();
    let acquirers = index
        .get(name)
        .ok_or_else(|| FetchPackageError::PackageNotFound)?;

    let mut errors = Vec::new();
    for acquirer in acquirers {
        match acquirer.latest() {
            Ok(info) => {
                return Ok(info);
            }
            Err(e) => {
                errors.push((acquirer.source_name(), e));
            }
        }
    }

    Err(FetchPackageError::FetchFailed(errors))
}
