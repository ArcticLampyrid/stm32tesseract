use std::time::Duration;

use crate::reqwest_unified_builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirrorType {
    Official,
    GhProxy,
}
static MIRROR_TYPE: std::sync::OnceLock<MirrorType> = std::sync::OnceLock::new();

fn is_outbound_to_china() -> bool {
    let urls = [
        "https://www.cloudflare.com/cdn-cgi/trace",
        "https://www.cloudflare-cn.com/cdn-cgi/trace",
        "https://www.cf-ns.com/cdn-cgi/trace",
    ];
    let client = reqwest_unified_builder::build_blocking();
    if let Ok(client) = client {
        for url in urls.iter() {
            let result = client
                .get(*url)
                .timeout(Duration::from_secs(10))
                .send()
                .ok()
                .and_then(|response| {
                    if response.status().is_success() {
                        response.text().ok().and_then(|body| {
                            if body.contains("loc=") {
                                return Some(body.contains("loc=CN"));
                            }
                            None
                        })
                    } else {
                        None
                    }
                });
            if let Some(result) = result {
                return result;
            }
        }
    }
    false
}

fn get_mirror_type() -> MirrorType {
    let result = *MIRROR_TYPE.get_or_init(|| {
        if !is_outbound_to_china() {
            return MirrorType::Official;
        }
        let client = reqwest_unified_builder::build_blocking();
        if let Ok(client) = client {
            // for China users, there may be a challenge to download the official resources
            // use proxy if available
            let use_proxy = client
                .get("https://ghp.ci/")
                .timeout(Duration::from_secs(10))
                .send()
                .ok()
                .map(|response| {
                    if response.status().is_success() || response.status().is_redirection() {
                        return Some(());
                    }
                    None
                })
                .is_some();
            if use_proxy {
                return MirrorType::GhProxy;
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
            return format!("https://ghp.ci/{}", url);
        }
        if url.contains("://raw.githubusercontent.com/") {
            // raw content url
            return format!("https://ghp.ci/{}", url);
        }
    }
    url
}
