use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CampanulaPoWEntity {
    pub prefix: String,
    pub difficulty: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CampanulaAssetEntity {
    pub name: String,
    pub download_url: String,
    pub pow: Option<CampanulaPoWEntity>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct CampanulaPkgInfoEntity {
    pub name: String,
    pub version_name: String,
    pub version_code: String,
    pub assets: Vec<CampanulaAssetEntity>,
}
