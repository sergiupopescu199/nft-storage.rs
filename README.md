# nft-storage.rs
This crate is a wrapper around the nft storage REST API with some additions, it use `tokio` as async runtime and all the results are in `serde_json::Value` format.

Examples are present in the `examples` directory, to make them work you must create an [nft storage](https://nft.storage/) account and generate an API Key

### Create an NFT

When storing an nft first of all it upload the desired file to nft storage, store in memory the file’s cid and then create another file `metadata.json`  there the file cid previously created is saved and also the nft name and it’s description
Is not ERC-1155 compatible NFT in some way but is a very flexible alternative, you can upload every type of file not just videos and images

```rust
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
    // read a file in order to have a Vec<u8>
    let file = std::fs::read("hello.txt")?;
    // store an nft
    let store_nft: Value = nft_storage
        .store_nft(file, "hello.txt", "My NFT name", "My NFT description")
        .await?;
    println!("{}", to_string_pretty(&store_nft)?);

    Ok(())
}

```

### List NFTs

List all stored nfts
`before` is the timestamp it uses this format `2020-07-27T17:32:28Z` or is possible to get the timestamp directly from the response in `value.created`. 
`limit` is the amount of files to show in a single request

both  `before` and `limit` are optional but `only_metadata` is required

```json
{
  "ok": true,
  "value": [
    {
      "cid": "bafybeiflbavrum45ekg5qxbecvpn5bfcvuk45txcmgsabfebtkv44cn6vq",
      "created": "2021-12-02T08:52:33.461+00:00",
      "type": "image/*",
      "scope": "session",
      "files": [],
      "size": 1147081,
      "pin": {
        "cid": "bafybeiflbavrum45ekg5qxbecvpn5bfcvuk45txcmgsabfebtkv44cn6vq",
        "created": "2021-12-02T08:52:33.461+00:00",
        "size": 1147081,
        "status": "queued"
      },
      "deals": []
    }
  ]
}
```



```rust
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
        .list_all_stored_nft(None, None)
        // .list_all_stored_nft(None, Some("100"))
        // .get_nft("bafybeibo4rijplqlv6o6j7jcftx4ckgzjv43jd2whqeluc5dnxslutsdda")
        .await?;
    println!("{}", to_string_pretty(&list_nft)?);

    Ok(())
}

```

### Delete NFT

You can delete an nft/file by cid or you can delete all files/nfts

```rust
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
        // .delete_nft("bafybeibo4rijplqlv6o6j7jcftx4ckgzjv43jd2whqeluc5dnxslutsdda")
        .delete_all_nft()
        .await?;
    println!("{}", to_string_pretty(&list_nft)?);

    Ok(())
}

```

### Upload a file

You can upload a file, it will not generate a `metadata.json` file

```rust
use nft_storage::NftStorage;
use anyhow::Result;
use serde_json::Value;
use reqwest::multipart::{Form, Part}
    
#[tokio::main]
async fn main() -> Result<()> {
 	// provide the url and as second argument the token generated from nft storage dashboard
    let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    // read a file in order to have a Vec<u8> the same from a form-data
    let file = std::fs::read("my_file.jpg")?;
    // delete nft
    let upload_file: Value  = nft_storage.upload_file(file).await?;
    
    Ok(())
```

