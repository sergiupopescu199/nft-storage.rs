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
    // it is possible to put more than one file name
    let file_names = vec!["nft.txt"];
    // read file, it is possible to read more files
    let file = std::fs::read(file_names[0])?;
    // collect file, it is possible to add more files, when recieving a stream of bytes from form-data it
    // often can send to us more than one file so all file bytes can be stored in this vec, the same is for file names vec
    let file_bytes_vec = vec![file];

    // store one or multiple nfts in a directory
    let store_file: StoreNftResponse = nft_storage
        .store_nft_in_directory(
            file_bytes_vec,
            file_names,
            "My nft name",
            "My nft description",
        )
        .await?;

    println!("{}", to_string_pretty(&store_file)?);

    Ok(())
}
