use anyhow::Result;
use nft_storage::{types::StoreNftResponse, NftStorage};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<()> {
    // provide the url and as second argument the token generated from nft storage dashboard
    let nft_storage = NftStorage::new(
        "https://api.nft.storage",
        "token generated from nft storage",
    );
    // read file
    let file = std::fs::read("hello.txt")?;
    // upload a file
    let store_file: StoreNftResponse = nft_storage.upload_file(file).await?;
    println!("{}", to_string_pretty(&store_file)?);

    Ok(())
}
