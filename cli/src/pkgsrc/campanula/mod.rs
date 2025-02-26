//! Campanula --- Blessings From Afar  
//! 风铃草 —— 来自远方的祝福  
//!
//! Where sunlight fractures in prismatic streams,  
//! Campanula's gaze bends through astral seams—  
//! Each refracted beam a cipher of distant wishes,  
//! Zephyr-carved shadows sing what eyes are missing.  
//! 当阳光在棱镜中破碎，  
//! 风铃草的目光透过星隙漫游——  
//! 每束折光都是远方祝福的密码，  
//! 西风包裹暗影，吟唱目之未睹。  
//!
//! Campanula is codename of infrastructure that are used to deliver
//! packages to users from our mirrors instead of directly from the
//! upstream.
//! Such mechanism is designed to provide a more stable and reliable
//! package fetching experience, for the upstream may be blocked or
//! throttled in some regions.

mod asset_acquirer;
mod entity;
mod pkg_acquirer;
use super::pkg_index::PackageIndex;
use crate::reqwest_instance;
pub use asset_acquirer::CampanulaAssetAcquirer;
pub use pkg_acquirer::CampanulaPkgAcquirer;

pub fn index_of_campanula(base_url: &str) -> PackageIndex {
    let client = reqwest_instance::blocking_client();
    let url = if base_url.ends_with('/') {
        format!("{}v1/index", base_url)
    } else {
        format!("{}/v1/index", base_url)
    };
    let acquirers = client
        .get(url)
        .header("Accept", "text/plain")
        .send()
        .and_then(|response| response.text())
        .map(|text| {
            text.lines()
                .map(|line| CampanulaPkgAcquirer::new(base_url.to_string(), line.to_string()))
                .collect::<Vec<_>>()
        });
    let mut index = PackageIndex::new();
    if let Ok(acquirers) = acquirers {
        for acquirer in acquirers {
            let name = acquirer.name().to_string();
            index.insert(name, Box::new(acquirer) as _);
        }
    } else {
        println!(
            "Failed to fetch package index from Campanula server: {}",
            base_url
        );
    }
    index
}
