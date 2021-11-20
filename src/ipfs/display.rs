use futures::TryStreamExt;
use ipfs_api::{IpfsApi, IpfsClient};
use std::io::{self, Write};

/// Fetch file from IPFS Using CID and Display in Stdout
pub async fn display_file_content(cid: &str) {
    let client = IpfsClient::default();
    // Fetch file content from IPFS
    match client
        .cat(&cid)
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
    {
        Ok(res) => {
            // Display File Content
            let out = io::stdout();
            let mut out = out.lock();

            out.write_all(&res).unwrap();
        }
        Err(e) => eprintln!("Error getting file: {}.", e),
    };
}
