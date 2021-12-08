use anyhow::Result;
use nft_storage::NftStorage;
use serde_json::{to_string_pretty, Value};

#[tokio::main]
async fn main() -> Result<()> {
    // provide the url and as second argument the token generated from nft storage dashboard
    let nft_storage = NftStorage::new("https://api.nft.storage", "oken generated from nft storage");
    // store an nft
    let list_nft: Value = nft_storage
        .list_all_stored_nft(None, None, false)
        // .list_all_stored_nft(None, Some("100"), true)
        // .get_nft("bafybeibjt6afd4u7or3olgfhy7cc2t2zfpf436w6limuiyp3347t23andy")
        .await?;
    println!("{}", to_string_pretty(&list_nft)?);

    Ok(())
}
