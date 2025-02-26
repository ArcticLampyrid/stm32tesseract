use super::entity::{CampanulaAssetEntity, CampanulaPoWEntity};
use crate::pkgsrc::PackageAsset;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use sha1::{Digest, Sha1};
use url::form_urlencoded::byte_serialize;

fn solve_pow(params: &CampanulaPoWEntity) -> String {
    let prefix = &params.prefix;
    let difficulty = params.difficulty;
    let mut nonce = vec![0];
    loop {
        let nonce_str = URL_SAFE_NO_PAD.encode(&nonce);
        let input = format!("{}{}", prefix, nonce_str);

        let mut hasher = Sha1::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();

        let mut n_bits: u32 = 0;
        'byte_loop: for byte in hash.iter() {
            let mut byte = *byte;
            for _ in 0..8 {
                if byte & 0x80 != 0 {
                    break 'byte_loop;
                }
                n_bits += 1;
                byte <<= 1;
            }
        }
        if n_bits >= difficulty {
            return input;
        }

        let mut carry = false;
        for i in nonce.iter_mut() {
            if *i == 255 {
                *i = 0;
                carry = true;
            } else {
                *i += 1;
                carry = false;
                break;
            }
        }
        if carry {
            nonce.push(0);
        }
    }
}

pub struct CampanulaAssetAcquirer {
    entity: CampanulaAssetEntity,
    download_url_cache: std::sync::OnceLock<String>,
}

impl CampanulaAssetAcquirer {
    pub fn new(entity: CampanulaAssetEntity) -> Self {
        Self {
            entity,
            download_url_cache: std::sync::OnceLock::new(),
        }
    }
}

impl PackageAsset for CampanulaAssetAcquirer {
    fn name(&self) -> &str {
        &self.entity.name
    }

    fn download_url(&self) -> &str {
        if let Some(pow) = &self.entity.pow {
            self.download_url_cache
                .get_or_init(|| {
                    let solution = solve_pow(pow);
                    let solution_url_iter = byte_serialize(solution.as_bytes());
                    let mut solution_url = String::new();
                    for s in solution_url_iter {
                        solution_url.push_str(s);
                    }
                    if self.entity.download_url.contains('?') {
                        format!("{}&pow={}", &self.entity.download_url, solution_url)
                    } else {
                        format!("{}?pow={}", &self.entity.download_url, solution_url)
                    }
                })
                .as_str()
        } else {
            &self.entity.download_url
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_pow() {
        let params = CampanulaPoWEntity {
            prefix: "AStfv2crX79niClR3UZmG0p_i2YuqvDtwXN0bTMydGVzc2VyYWN0fHYwLjEuNnxzdG0zMnRlc3NlcmFjdC1ndWlfMC4xLjYtMV9hbWQ2NC5kZWI=$cqI4IsUUD0U6Lx1LH91eg678cgM=$".to_string(),
            difficulty: 16,
        };
        let computed = solve_pow(&params);
        println!("computed: {}", computed);
        let mut hasher = Sha1::new();
        hasher.update(computed.as_bytes());
        let hash = hasher.finalize();
        let hash_hex = format!("{:x}", hash);
        println!("hash: {}", hash_hex);
        assert_eq!(&hash_hex[..4], "0000");
    }
}
