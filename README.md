# ipfs-smart-contract
Upload a file to IPFS and then store the CID in a smart contract.

# VSCode Extensions Used:
1. -> Name: Better TOML
    Id: bungcip.better-toml
    Description: Better TOML Language support
    Version: 0.3.2
    Publisher: bungcip
    VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml
2. -> Name: Rust
    Id: rust-lang.rust
    Description: Rust for Visual Studio Code (powered by Rust Language Server/Rust Analyzer). Provides lints, code completion and navigation, formatting and more.
    Version: 0.7.8
    Publisher: The Rust Programming Language 
    VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust
3. -> Name: solidity
    Id: juanblanco.solidity
    Description: Ethereum Solidity Language for Visual Studio Code
    Version: 0.0.136
    Publisher: Juan Blanco
    VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=JuanBlanco.solidity

# Pre-Requisite
1. Install IPFS:
    -> https://docs.ipfs.io/install/command-line/#system-requirements
    -> https://docs.ipfs.io/how-to/command-line-quick-start/#prerequisites
    -> verify using `ipfs --version`
2. Install Ganache:
    -> npm install ganache@beta --global
    -> verify using `ganache-cli --version`
3. Set env CARGO_MANIFEST_DIR="../"

# How to Compile and Test:
1. `cargo check`
2. `cargo build --release`
3. `cargo test`

# How to Run:
```
USAGE:
    ipfs_smart_contract --file-path <File Path>
```

Help Menu: `ipfs_smart_contract --help`

Sample test script and other program resources are kept in test-bed folder.
Execute run.sh to upload test-data.txt file to ipfs, store and deploy cid using smart contract.

Note: Keep IPFS Daemon running in background using `ipfs daemon`

# Download Release V1.0.0: 
Release contains:
1. ipfs_smart_contract binary
2. Sample run script

# Reference Reading Material:
1. https://ethereumdev.io/deploying-your-first-smart-contract/
2. https://docs.openzeppelin.com/learn/deploying-and-interacting
3. https://docs.rs/ethers-contract/0.5.4/ethers_contract/struct.Contract.html
4. https://github.com/gakonst/ethers-rs
5. https://github.com/rs-ipfs/rust-ipfs
6. https://crates.io/crates/ipfs-api