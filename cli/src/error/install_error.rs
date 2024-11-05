use std::{io, process::ExitStatus};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstallError {
    #[error("failed to download: {0:?}")]
    HttpFetchFailed(#[from] reqwest::Error),
    #[error("http status: {0:?}")]
    HttpStatusError(reqwest::StatusCode),
    #[error("metadata error")]
    MetadataError(),
    #[error("io failed: {0:?}")]
    IOFailed(#[from] io::Error),
    #[error("not supported to install in current arch")]
    ArchNotSupported(),
    #[error("not supported to install in current os")]
    OsNotSupported(),
    #[error("failed to find supported package manager")]
    SupportedPackageManagerNotFound(),
    #[error("external program failed: {0:?}")]
    ExternalProgramFailed(ExitStatus),
    #[error("invalid zip archive: {0:?}")]
    InvalidZipArchive(#[from] zip::result::ZipError),
    #[error("DownloadError: {0}")]
    DownloadError(#[from] crate::download_manager::DownloadError),
    #[error("unknown error")]
    #[allow(dead_code)]
    UnknownError(),
}
