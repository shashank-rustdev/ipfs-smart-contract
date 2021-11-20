use ipfs_api::{IpfsApi, IpfsClient};
use std::fs::File;

/// Upload File to IPFS
pub async fn upload_file(path: &str) -> String {
    let client = IpfsClient::default();
    let data = File::open(path).expect(&format!("Could not read source file at {}.", path));
    // Add file to IPFS and return CID
    match client.add(data).await {
        Ok(res) => res.hash,
        Err(e) => panic!("Error uploading {}: {}", path, e),
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_ipfs_upload_success() {
        let path = "test-bed/test-data.txt";
        let cid = crate::ipfs::upload_file(path).await;
        assert_eq!("QmPEJ9wV2zMmZp7ianWU2gXkkVEay6eDtycqnBDdy3yySY", cid);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_ipfs_upload_fail() {
        let path = "test-bed/does-not-exist.txt";
        crate::ipfs::upload_file(path).await;
    }
}
