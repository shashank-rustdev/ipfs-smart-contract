#![cfg_attr(
    feature = "cargo-clippy",
    deny(
        clippy::unwrap_used
    )
)]
use contract::deploy_contract;
use ipfs::upload_file;

mod configuration_parameters;
mod contract;
mod ipfs;

#[tokio::main]
async fn main() {
    // 1. Get Program Parameters
    let app_name = "ipfs_smart_contract";
    let config_param = configuration_parameters::get_configuration_parameters(app_name);

    // 2. Upload File to IPFS
    let cid = upload_file(config_param.file_path()).await;
    println!("File Successfully Uploaded to IPFS.\nCID: {:?}.", cid);

    // 3. Deploy Smart Contract
    match deploy_contract(cid).await {
        Ok(addr) => {
            println!("IPFS Smart Contract Successfully Deployed at: {:?}.", addr);
        }
        Err(e) => println!("IPFS Smart Contract Could not be Deployed: {:?}.", e),
    }
}
