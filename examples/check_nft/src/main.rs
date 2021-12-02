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
    // check if nft is stored on nft storage
    let is_stored_on_nft_storage: Value = nft_storage
        .check_nft("bafybeiflbavrum45ekg5qxbecvpn5bfcvuk45txcmgsabfebtkv44cn6vq")
        .await?;
    println!("{}", to_string_pretty(&is_stored_on_nft_storage)?);

    Ok(())
}
