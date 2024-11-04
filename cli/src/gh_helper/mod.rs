use crate::error::InstallError;
mod mirror_election;
pub use mirror_election::elect_mirror;

pub fn get_latest_release_url<F>(
    client: &reqwest::blocking::Client,
    owner: &str,
    repo: &str,
    mut filter: F,
) -> Result<String, InstallError>
where
    F: FnMut(&str) -> bool,
{
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
    let response = client.get(url).send()?;
    if response.status().is_success() {
        let response_data: serde_json::Value = response.json()?;
        if let Some(releases_data) = response_data.as_array() {
            for release_data in releases_data.iter() {
                if release_data["prerelease"].as_bool() == Some(true) {
                    // Skip pre-release
                    continue;
                }
                if let Some(asserts_data) = release_data["assets"].as_array() {
                    let url_target_asset = asserts_data.iter().find_map(|assert_data| {
                        if let Some(assert_name) = assert_data["name"].as_str() {
                            if filter(assert_name) {
                                return assert_data["browser_download_url"].as_str();
                            }
                        }
                        None
                    });
                    if let Some(url_target_asset) = url_target_asset {
                        return Ok(url_target_asset.to_string());
                    }
                }
            }
        }
        Err(InstallError::MetadataError())
    } else {
        Err(InstallError::HttpStatusError(response.status()))
    }
}

pub fn get_latest_release_url_with_fallback<F>(
    client: &reqwest::blocking::Client,
    owner: &str,
    repo: &str,
    filter: F,
    fallback_url: &str,
) -> String
where
    F: FnMut(&str) -> bool,
{
    match get_latest_release_url(client, owner, repo, filter) {
        Ok(url) => url,
        Err(_) => {
            println!("Failed to get latest release url of {}/{}", owner, repo);
            println!("Using fallback url");
            fallback_url.to_string()
        }
    }
}
