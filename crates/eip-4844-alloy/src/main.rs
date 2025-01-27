//! Example of using the HTTP provider to get the latest block number.

use alloy_consensus::{
    BlobTransactionSidecar, ReceiptWithBloom, SidecarBuilder, TxEip1559, TxEip2930, TxEip4844,
    TxEip4844Variant, TxEip4844WithSidecar, TxLegacy,
};
use alloy_network::Ethereum;
use alloy_node_bindings::Anvil;
use alloy_primitives::{Address, Bloom, Bytes, Log, Signature, TxHash, B256, U128, U256, U64};
use alloy_provider::{HttpProvider, Provider};
use alloy_rlp::{Decodable, Encodable};
use alloy_rpc_client::RpcClient;
use alloy_rpc_types::{
    request::TransactionRequest, AccessList, Signature as RpcSignature,
    Transaction as RpcTransaction,
};
use alloy_signer_wallet::LocalWallet;
use eyre::Result;
// use kzg::{KzgCommitment, KzgProof, KzgSettings};

use c_kzg::{Blob, Bytes48, KzgSettings, BYTES_PER_BLOB, BYTES_PER_COMMITMENT, BYTES_PER_PROOF};
// use alloy_eips::eip4844::Blob;
use std::{fs, path::PathBuf};

async fn send_blob_txns() -> Result<()> {
    let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;

    // Create a provider.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = HttpProvider::<Ethereum>::new_http(rpc_url);

    // Create two users, Alice and Bob.
    let alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    let mut builder: SidecarBuilder = SidecarBuilder::new();
    // ingest block data
    builder.ingest(b"dummy blob");
    // build the sidecar with default KZG settings after all ingestion is finished
    // let sidecar: BlobTransactionSidecar = builder.build();
    // Create a transaction to transfer 1 wei from Alice to Bob.
    // let data = BlobReader::readBlobCall::new(()).abi_encode();
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join("blob.json");
    let json_content = fs::read_to_string(file_path).expect("Failed to read the blob data file");
    let json_value: serde_json::Value =
        serde_json::from_str(&json_content).expect("Failed to deserialize JSON");
    let blobs: Vec<Blob> = vec![Blob::from_hex(
        json_value
            .get("data")
            .unwrap()
            .as_str()
            .expect("Data is not a valid string"),
    )
    .unwrap()];
    // let blob_sidecar = BlobTransactionSidecar {
    //     blobs: sidecar
    //         .blobs
    //         .into_iter()
    //         .map(|b| c_kzg::Blob::from_bytes(b.as_slice()).unwrap())
    //         .collect(),
    //     commitments: sidecar
    //         .commitments
    //         .into_iter()
    //         .map(|c| c_kzg::Bytes48::from_bytes(c.as_slice()).unwrap())
    //         .collect(),
    //     proofs: sidecar
    //         .proofs
    //         .into_iter()
    //         .map(|p| c_kzg::Bytes48::from_bytes(p.as_slice()).unwrap())
    //         .collect(),
    // };
    // let tx: TransactionRequest = TransactionRequest {
    //     to: Some(bob),
    //     input: TransactionInput::new(),
    //     sidecar: Some(sidecar),
    //     ..Default::default()
    // };
    // // let tx = TransactionRequest::default()
    // //     .from(alice)
    // //     .value(U256::from(1))
    // //     .to(Some(bob));

    // // Send the transaction and wait for the receipt.
    // let pending_tx = provider.send_transaction(tx).await?;

    // println!("Pending transaction...{:?}", pending_tx.tx_hash());

    // // Wait for the transaction to be included.
    // let receipt = pending_tx.get_receipt().await?;

    // println!(
    //     "Transaction included in block: {:?}",
    //     receipt
    //         .block_number
    //         .expect("Failed to get block number")
    //         .to_string()
    // );

    // // assert_eq!(receipt.from, alice);
    // assert_eq!(receipt.to, Some(bob));

    Ok(())
}
async fn send_simple_txns() -> Result<()> {
    let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;

    // Create a provider.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = HttpProvider::<Ethereum>::new_http(rpc_url);

    // Create two users, Alice and Bob.
    let alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    // Create a transaction to transfer 1 wei from Alice to Bob.
    let tx = TransactionRequest::default()
        .from(alice)
        .value(U256::from(1))
        .to(Some(bob));

    // Send the transaction and wait for the receipt.
    let pending_tx = provider.send_transaction(tx).await?;

    println!("Pending transaction...{:?}", pending_tx.tx_hash());

    // Wait for the transaction to be included.
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block: {:?}",
        receipt
            .block_number
            .expect("Failed to get block number")
            .to_string()
    );

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    send_simple_txns().await?; // Set up the HTTP transport which is consumed by the RPC client.
                               // send_blob_txns().await?;
    let rpc_url = "https://sepolia.infura.io/v3/3753abe17c124d088f4e68d58f257452".parse()?;

    // Create the RPC client.
    let rpc_client = RpcClient::new_http(rpc_url);

    // Provider can then be instantiated using the RPC client, HttpProvider is an alias
    // RootProvider. RootProvider requires two generics N: Network and T: Transport
    let provider = HttpProvider::<Ethereum>::new(rpc_client);

    let chain_id = provider.get_chain_id().await?;
    // Get latest block number.
    // let latest_block = provider.get_block_number().await?;
    // let wallet: LocalWallet = "cf43d32585172619dbc5b46b20560e96c563c01f0e55e1b36837d8441ca36a8a"
    //     .parse::<LocalWallet>()?
    //     .with_chain_id(Some(11155111));
    // println!("Latest block number: {latest_block}");

    Ok(())
}
