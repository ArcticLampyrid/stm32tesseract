use super::{pkg_index::PackageIndex, GenericPackageAsset, PackageAcquirer, PackageAsset};
use crate::reqwest_instance;

pub struct GitHubReleaseAcquirer {
    name: String,
    qualified_repo: String,
}

struct GitHubReleaseAsset {
    name: String,
    download_url: String,
}

impl PackageAsset for GitHubReleaseAsset {
    fn name(&self) -> &str {
        &self.name
    }

    fn download_url(&self) -> &str {
        &self.download_url
    }
}

impl GitHubReleaseAcquirer {
    pub fn new(name: &str, qualified_repo: &str) -> Self {
        Self {
            name: name.to_string(),
            qualified_repo: qualified_repo.to_string(),
        }
    }
}

impl PackageAcquirer for GitHubReleaseAcquirer {
    fn latest(&self) -> super::GenericResult<super::PackageInfo> {
        let client = reqwest_instance::blocking_client();
        let url = format!(
            "https://api.github.com/repos/{}/releases/latest",
            self.qualified_repo
        );
        let response = client
            .get(url)
            .header("Accept", "application/vnd.github.v3+json")
            .send()?;
        if response.status().is_success() {
            let response_data: serde_json::Value = response.json()?;
            if let Some(response_data) = response_data.as_object() {
                let version_name = response_data["tag_name"].as_str().unwrap_or_default();
                let version_code = response_data["target_commitish"]
                    .as_str()
                    .unwrap_or_default();
                let mut assets = Vec::<GenericPackageAsset>::new();
                if let Some(asserts) = response_data["assets"].as_array() {
                    for asset in asserts {
                        let name = asset["name"].as_str().unwrap_or_default();
                        let download_url =
                            asset["browser_download_url"].as_str().unwrap_or_default();
                        assets.push(Box::new(GitHubReleaseAsset {
                            name: name.to_string(),
                            download_url: download_url.to_string(),
                        }));
                    }
                }
                Ok(super::PackageInfo {
                    name: self.name.clone(),
                    version_name: version_name.to_string(),
                    version_code: version_code.to_string(),
                    assets,
                })
            } else {
                Err("invalid response data".into())
            }
        } else {
            Err(format!("http error: {}", response.status()).into())
        }
    }

    fn source_name(&self) -> String {
        format!("gh-release:{}", self.qualified_repo)
    }
}

fn insert_gh_acquirer(index: &mut PackageIndex, name: &str, qualified_repo: &str) {
    index.insert(
        name.to_string(),
        Box::new(GitHubReleaseAcquirer::new(name, qualified_repo)),
    );
}

pub fn index_of_github_release() -> super::PackageIndex {
    let mut index = PackageIndex::new();
    insert_gh_acquirer(
        &mut index,
        "stm32tesseract",
        "ArcticLampyrid/stm32tesseract",
    );
    insert_gh_acquirer(
        &mut index,
        "arm-none-eabi-gcc-xpack",
        "xpack-dev-tools/arm-none-eabi-gcc-xpack",
    );
    insert_gh_acquirer(&mut index, "cmake", "Kitware/CMake");
    insert_gh_acquirer(&mut index, "ninja", "ninja-build/ninja");
    insert_gh_acquirer(&mut index, "openocd", "openocd-org/openocd");
    index
}
