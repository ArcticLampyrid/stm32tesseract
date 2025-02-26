use crate::{
    pkgsrc::{GenericResult, PackageAcquirer, PackageInfo},
    reqwest_instance,
};

use super::{entity::CampanulaPkgInfoEntity, CampanulaAssetAcquirer};

pub struct CampanulaPkgAcquirer {
    base_url: String,
    name: String,
}

impl CampanulaPkgAcquirer {
    pub fn new(base_url: String, name: String) -> Self {
        Self { base_url, name }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl PackageAcquirer for CampanulaPkgAcquirer {
    fn latest(&self) -> GenericResult<PackageInfo> {
        let client = reqwest_instance::blocking_client();
        let url = if self.base_url.ends_with('/') {
            format!("{}/v1/package/{}", self.base_url, self.name)
        } else {
            format!("{}/v1/package/{}", self.base_url, self.name)
        };
        client
            .get(url)
            .header("Accept", "application/json")
            .send()
            .and_then(|response| response.json::<CampanulaPkgInfoEntity>())
            .map(|entity| {
                let assets = entity
                    .assets
                    .into_iter()
                    .map(|asset| Box::new(CampanulaAssetAcquirer::new(asset)) as _)
                    .collect();
                PackageInfo {
                    name: entity.name,
                    version_name: entity.version_name,
                    version_code: entity.version_code,
                    assets,
                }
            })
            .map_err(|err| Box::new(err) as _)
    }

    fn source_name(&self) -> String {
        "Campanula".to_string()
    }
}
