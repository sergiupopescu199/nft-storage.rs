use anyhow::Result;
use nft_storage::{types::DeleteNftResponse, NftStorage};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<()> {
    // provide the url and as second argument the token generated from nft storage dashboard
    let nft_storage = NftStorage::new(
        "https://api.nft.storage",
        "token generated from nft storage",
    );
    // store an nft
    let deleted_nft: DeleteNftResponse = nft_storage
        // .delete_nft("bafybeibo4rijplqlv6o6j7jcftx4ckgzjv43jd2whqeluc5dnxslutsdda")
        .delete_all_nft()
        .await?;
    println!("{}", to_string_pretty(&deleted_nft)?);

    Ok(())
}
