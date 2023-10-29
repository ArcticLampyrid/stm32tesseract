use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CProjectReaderError {
    #[error("io failed: {0:?}")]
    IOFailed(#[from] io::Error),
    #[error("invalid xml: {0:?}")]
    InvalidXML(#[from] sxd_document::parser::Error),
    #[error("xpath failed: {0:?}")]
    XPathFailed(#[from] sxd_xpath::Error),
}
