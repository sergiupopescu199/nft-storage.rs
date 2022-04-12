use anyhow::Result;
use nft_storage::{ListNftResponse, NftStorage};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<()> {
    // provide the url and as second argument the token generated from nft storage dashboard
    let nft_storage = NftStorage::new(
        "https://api.nft.storage",
        "token generated from nft storage",
    );
    // store an nft
    let list_nft: ListNftResponse = nft_storage
        .list_all_stored_nft(None, None, true)
        // .list_all_stored_nft(None, Some("100"), true)
        // .get_nft("bafybeibjt6afd4u7or3olgfhy7cc2t2zfpf436w6limuiyp3347t23andy")
        .await?;
    println!("{}", to_string_pretty(&list_nft)?);

    Ok(())
}
