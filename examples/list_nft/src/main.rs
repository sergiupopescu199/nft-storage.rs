use anyhow::Result;
use nft_storage::NftStorage;
use serde_json::{to_string_pretty, Value};

#[tokio::main]
async fn main() -> Result<()> {
    // provide the url and as second argument the token generated from nft storage dashboard
    let nft_storage = NftStorage::new(
        "https://api.nft.storage",
        "token generated from nft storage",
    );
    // store an nft
    let list_nft: Value = nft_storage
        // .list_all_stored_nft(None, None)
        // .list_all_stored_nft(None, Some("100"))
        .get_nft("bafkreiavysxc6slat577h6xe6immtimm3qttdzaozszhoi4usgsyibohha")
        .await?;
    println!("{}", to_string_pretty(&list_nft)?);

    Ok(())
}
