use anyhow::Result;
use nft_storage::{NftStorage, StoreNftResponse};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<()> {
    // provide the url and as second argument the token generated from nft storage dashboard
    let nft_storage = NftStorage::new(
        "https://api.nft.storage",
        "token generated from nft storage",
    );
    // read a file in order to have a Vec<u8> the same from a form-data
    let file = std::fs::read("hello.txt")?;
    // store an nft
    let store_nft: StoreNftResponse = nft_storage
        .store_nft(file, "My NFT name", "My NFT description")
        .await?;
    println!("{}", to_string_pretty(&store_nft)?);

    Ok(())
}
