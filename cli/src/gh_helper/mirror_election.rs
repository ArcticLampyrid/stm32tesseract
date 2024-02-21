use std::time::Duration;

use crate::reqwest_unified_builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirrorType {
    Official,
    GhProxy,
}
static MIRROR_TYPE: std::sync::OnceLock<MirrorType> = std::sync::OnceLock::new();

fn get_mirror_type() -> MirrorType {
    let result = *MIRROR_TYPE.get_or_init(|| {
        let client = reqwest_unified_builder::build_blocking();
        if let Ok(client) = client {
            {
                // check location
                // for non-China users, use official resources
                let response = client
                    .get("https://www.cloudflare.com/cdn-cgi/trace")
                    .timeout(Duration::from_secs(10))
                    .send();
                if let Ok(response) = response {
                    if response.status().is_success() {
                        if let Ok(body) = response.text() {
                            if !body.contains("loc=CN") {
                                return MirrorType::Official;
                            }
                        }
                    }
                }
            }
            // for China users, there may be a challenge to download the official resources
            {
                // check if ghproxy is available
                let response = client
                    .get("https://mirror.ghproxy.com/")
                    .timeout(Duration::from_secs(10))
                    .send();
                if let Ok(response) = response {
                    if response.status().is_success() {
                        return MirrorType::GhProxy;
                    }
                }
            }
        }
        MirrorType::Official
    });
    if result != MirrorType::Official {
        println!("Info: access GitHub via {:?}", result);
    }
    result
}

pub fn elect_mirror(url: String) -> String {
    let mirror_type = get_mirror_type();
    if mirror_type == MirrorType::GhProxy {
        if url.contains("://github.com/") && url.contains("/releases/download/") {
            // release download url
            return format!("https://mirror.ghproxy.com/{}", url);
        }
        if url.contains("://raw.githubusercontent.com/") {
            // raw content url
            return format!("https://mirror.ghproxy.com/{}", url);
        }
    }
    url
}
