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
    let file_names = vec!["hello.txt", "ciao.txt"];
    // read multiple files
    let file = std::fs::read(file_names[0])?;
    let file2 = std::fs::read(file_names[1])?;
    // collect the files in a vec
    let file_bytes_vec = vec![file, file2];

    // upload file in a directory
    let store_file: Value = nft_storage
        .upload_file_in_directory(file_bytes_vec, file_names)
        .await?;

    println!("{}", to_string_pretty(&store_file)?);

    Ok(())
}
