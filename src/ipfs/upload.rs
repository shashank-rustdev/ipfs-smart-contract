use ipfs_api::{IpfsApi, IpfsClient};
use std::fs::File;

/// Upload File to IPFS
pub async fn upload_file(path: &str) -> String {
    let client = IpfsClient::default();
    let data = File::open(path).expect(&format!("Could not read source file at {}.", path));
    // Add file to IPFS and return CID
    match client.add(data).await {
        Ok(res) => res.hash,
        Err(e) => panic!("Error adding {}: {}", path, e),
    }
}
