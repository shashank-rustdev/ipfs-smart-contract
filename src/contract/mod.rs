use ethers::{prelude::*, utils::Ganache};
use std::{convert::TryFrom, path::Path, sync::Arc, time::Duration};

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    SimpleContract,
    "src/contract/contract_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

/// Deploy smart contract to Ganace Local Env
pub async fn deploy_contract(cid: String) -> Result<H160, String> {
    // launch ganache
    let ganache = Ganache::new().spawn();

    // set the path to the contract, `CARGO_MANIFEST_DIR` points to the directory containing the
    // manifest of `ethers`. which will be `../` relative to this file
    let source = Path::new(&env!("CARGO_MANIFEST_DIR")).join("src/contract/contract.sol");
    let compiled = match Solc::default().compile_source(source) {
        Ok(res) => res,
        Err(e) => return Err(format!("Could not compile contracts: {}", e)),
    };
    let (abi, bytecode, _runtime_bytecode) = match compiled.find("IpfsStorage") {
        Some(res) => res.into_parts_or_default(),
        None => return Err("Could not find contract.".to_string()),
    };

    // instantiate wallet
    let wallet: LocalWallet = ganache.keys()[0].clone().into();

    // connect to the network
    let provider = match Provider::<Http>::try_from(ganache.endpoint()) {
        Ok(res) => res.interval(Duration::from_millis(10u64)),
        Err(e) => return Err(format!("Could not connect network: {}", e)),
    };

    // instantiate the client with the wallet
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // create a factory which will be used to deploy instances of the contract
    let factory = ContractFactory::new(abi, bytecode, client);

    // deploy it with the constructor arguments
    let contract = match factory.deploy(cid) {
        Ok(res) => match res.legacy().send().await {
            Ok(res) => res,
            Err(e) => return Err(format!("Cound not deploy the contract: {}", e)),
        },
        Err(e) => return Err(format!("Cound not prepare deployment transaction: {}", e)),
    };

    // return contract's address
    Ok(contract.address())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_sc_deploy() {
        let cid = "QmPEJ9wV2zMmZp7ianWU2gXkkVEay6eDtycqnBDdy3yySY".to_string();
        assert!(crate::contract::deploy_contract(cid).await.is_ok());
    }
}
