use std::str::FromStr;
use subxt::backend::rpc::RpcClient;
use subxt::{
    backend::legacy::rpc_methods::NumberOrHex, backend::legacy::LegacyRpcMethods,
    config::substrate::H256, OnlineClient, PolkadotConfig,
};
use tracing_subscriber::util::SubscriberInitExt;

#[subxt::subxt(runtime_metadata_path = "scale.metadata")]
mod src_chain {}

#[tokio::main]
async fn main() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap();

    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .finish()
        .try_init()
        .unwrap();

    let rpc = RpcClient::from_url("wss://polkadot-rpc.publicnode.com")
        .await
        .unwrap();

    let client = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc.clone())
        .await
        .unwrap();

    let legacy_rpc_client = LegacyRpcMethods::<PolkadotConfig>::new(rpc.clone());
    let b = legacy_rpc_client
        .chain_get_block_hash(Some(NumberOrHex::Number(24413043)))
        .await
        .unwrap()
        .unwrap();

    assert_eq!(
        b,
        H256::from_str("0x30f5cb1b5f604f743d725a134ecb114f7e2c94ab9699ccd329ba5d8810f1aa68")
            .unwrap()
    );

    let block = client.blocks().at(b).await.unwrap();
    let xts = block.extrinsics().await.unwrap();

    for xt in xts.iter() {
        println!(
            "{}::{} => xt_hash={:?}",
            xt.pallet_name().unwrap(),
            xt.variant_name().unwrap(),
            xt.hash(),
        );
    }
}
