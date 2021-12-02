mod error;
use anyhow::{anyhow, Result};
pub use error::NFTStorageError;
use reqwest::Client;
use serde_json::{json, Value};

/// NftStorage struct
pub struct NftStorage {
    /// reqwest client instance
    pub client: Client,
    /// nft storage rest api endpoint
    pub url: String,
    /// nft storage api token
    pub token: String,
}

/// Nft storage methods
impl NftStorage {
    /// Create a new instance of NftStorage
    ///
    /// The `url` is the url of the api which nftt storage is using for more information see `https://nft.storage/api-docs/`.
    ///
    /// The `token` is the jwt token generated from nft storage dashboard.
    /// ```
    /// use nft_storage::NftStorage;
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    ///
    /// 	Ok(())
    /// }
    /// ```
    ///
    pub fn new(url: &str, token: &str) -> NftStorage {
        NftStorage {
            client: Client::new(),
            url: url.to_string(),
            token: token.to_string(),
        }
    }

    /// List all nfts from nft storage
    /// `before` is used to return results created before provided timestamp `2021-12-01T08:52:33.461+00:00` or like this `2020-07-27T17:32:28Z` which is  and `limit` are the max records to return.
    ///
    /// the `only_metadata` option is used to return only the nft which contains the metadata.json file
    /// ```
    /// use nft_storage::NftStorage;
    /// use anyhow::Result;
    /// use serde_json::Value;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// list nfts only with metadata
    /// 	let list_nfts: Value  = nft_storage.list_all_stored_nft(None, None).await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    ///
    pub async fn list_all_stored_nft(
        &self,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> Result<Value, NFTStorageError> {
        // get the optional value or use an empty string
        let before = if let Some(value) = before {
            value.to_string()
        } else {
            "".to_string()
        };

        let limit = if let Some(value) = limit {
            value.to_string()
        } else {
            "".to_string()
        };
        // create the url to make the request
        let url = format!("{}/?before={}&limit={}", self.url, before, limit);
        // make the request to the nft storage api
        let response = self.client.get(url).bearer_auth(&self.token).send().await?;
        // check the response status if is in range from 200-299
        let status = response.status().is_success();
        let mut body = response.json::<Value>().await?;
        // if status is not success then return an error
        if !status {
            return Err(NFTStorageError::ApiError(body));
        }

        let final_filtered_list = body["value"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            // add additional convenience links to the filtered
            .map(|f| {
                let link_1 = format!("https://{}.ipfs.dweb.link", f["cid"].as_str().unwrap());
                let link_2 = format!("https://ipfs.io/ipfs/{}", f["cid"].as_str().unwrap());
                let link_3 = format!("ipfs://{}", f["cid"].as_str().unwrap());

                f["link"] = vec![link_1, link_2, link_3].into();

                f
            })
            .collect::<Vec<_>>();
        // modify the body with modified data, does include metadata.json and also all files
        body["value"] = serde_json::to_value(final_filtered_list)?;

        Ok(body)
    }

    /// Store an NFT on nft storage
    /// `file` is the file bytes recieved from a form-data and `file_name` is the filename of the file we want to create an nft
    ///
    /// `nft_name` is the nft name and `description` is the description of the nft
    ///
    /// ```
    /// use nft_storage::NftStorage;
    /// use anyhow::Result;
    /// use serde_json::Value;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// read a file in order to have a Vec<u8> the same from a form-data
    /// 	let file = std::fs::read("my_nft.jpg")?;
    /// 	// store an nft
    /// 	let store_nft: Value  = nft_storage.store_nft(file, "My NFT name", "My NFT description").await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    ///
    pub async fn store_nft(
        &self,
        file: Vec<u8>,
        nft_name: &str,
        description: &str,
    ) -> Result<Value, NFTStorageError> {
        // upload the file to nft storage, which is the actual file we want to create an nft
        let response = self.upload_file(file).await?;
        // get dir cid
        let cid = response["value"]["cid"]
            .as_str()
            .ok_or(anyhow!("Unable to parse json to get the string"))?;
        // create athe metadata form which will contain all files cid
        let metadata = json!({
            "name": nft_name,
            "description": description,
            "files": cid
        });
        // create the form-data instance for metadata.json
        let metadata_json_bytes = serde_json::to_vec(&metadata)?;
        // create the metadata.json which will contain the nft cids
        let response = self.upload_file(metadata_json_bytes).await?;

        Ok(response)
    }

    /// Delete an NFT
    /// `cid` is the ipfs hash, every file/nft has it's unique cid
    ///
    /// ```
    /// use nft_storage::NftStorage;
    /// use anyhow::Result;
    /// use serde_json::Value;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// delete nft
    /// 	let delete_nft: Value  = nft_storage.delete_nft("bafybeihflij24dndd6qo3aacbbysuzuygis7yurvrzxp3uk7bk5kfvfsdt").await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    ///
    pub async fn delete_nft(&self, cid: &str) -> Result<Value, NFTStorageError> {
        // create the url
        let url = format!("{}/{}", self.url, cid);
        // make the request to the nft storage api
        let response = self
            .client
            .delete(url)
            .bearer_auth(&self.token)
            .send()
            .await?;
        // check if the status of the request is in range of 200-299
        let status = response.status().is_success();
        let body = response.json::<Value>().await?;
        match status {
            true => Ok(body),
            false => Err(NFTStorageError::ApiError(body)),
        }
    }

    /// Delete all NFT
    ///
    /// It will fetch and delete all nfts
    /// ```
    /// use nft_storage::NftStorage;
    /// use anyhow::Result;
    /// use serde_json::Value;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// delete all nfts
    /// 	let delete_nfts: Value  = nft_storage.delete_all_nft().await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    ///
    pub async fn delete_all_nft(&self) -> Result<Value, NFTStorageError> {
        // create a loop to iterate and reqwest all nfts
        loop {
            // get first 10 nfts
            let nfts = self.list_all_stored_nft(None, None).await?;
            // check if ok is true this means the request was successfull and also check if the array is empty
            // if all is true break the loop this mean no nft are stored
            if nfts["ok"].as_bool().unwrap() == true && nfts["value"].as_array().unwrap().len() <= 0
            {
                break;
            }
            for e in nfts["value"]
                .as_array()
                .ok_or(anyhow!("Unable to parse json to get the vector "))?
            {
                println!("deleting CID: {}", e["cid"]);
                self.delete_nft(
                    e["cid"]
                        .as_str()
                        .ok_or(anyhow!("Unable to parse json to get the string"))?,
                )
                .await?;
                println!("NFT deleted: {}", e["cid"]);
            }
        }

        Ok(json!({
            "ok": true,
            "value": "all nfts are deleted"
        }))
    }

    /// Retrive an NFT
    ///
    /// It will fetch an nft from `cid`
    /// ```
    /// use nft_storage::NftStorage;
    /// use anyhow::Result;
    /// use serde_json::Value;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// get nft by cid
    /// 	let get_nft: Value  = nft_storage.get_nft("bafybeihflij24dndd6qo3aacbbysuzuygis7yurvrzxp3uk7bk5kfvfsfg").await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    ///
    pub async fn get_nft(&self, cid: &str) -> Result<Value, NFTStorageError> {
        let url = format!("{}/{}", self.url, cid);
        let response = self.client.get(url).bearer_auth(&self.token).send().await?;
        // check if the status of the request is in range of 200-299
        let status = response.status().is_success();
        let mut body = response.json::<Value>().await?;
        // add some convinient links
        let link_1 = format!(
            "https://{}.ipfs.dweb.link",
            body["value"]["cid"].as_str().unwrap()
        );
        let link_2 = format!(
            "https://ipfs.io/ipfs/{}",
            body["value"]["cid"].as_str().unwrap()
        );
        let link_3 = format!("ipfs://{}", body["value"]["cid"].as_str().unwrap());
        // modify the body with added links
        body["value"]["link"] = vec![link_1, link_2, link_3].into();

        match status {
            true => Ok(body),
            false => Err(NFTStorageError::ApiError(body)),
        }
    }

    /// Upload an arbitrary file on Nft storage
    ///
    /// It will upload an arbitrary file on ipfs backed up by nft storage and filecoin
    ///
    /// the max sise is around 30GB per file
    /// ```
    /// use nft_storage::NftStorage;
    /// use anyhow::Result;
    /// use serde_json::Value;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// read a file in order to have a Vec<u8> the same from a form-data
    /// 	let file = std::fs::read("my_nft.jpg")?;
    /// 	// delete nft
    /// 	let upload_file: Value  = nft_storage.upload_file(file).await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    pub async fn upload_file(&self, file: Vec<u8>) -> Result<Value, NFTStorageError> {
        let url = format!("{}/upload", self.url);
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.token)
            .body(file)
            .send()
            .await?;
        let status = response.status().is_success();
        // check if the status of the request is in range of 200-299
        let body = response.json::<Value>().await?;
        match status {
            true => Ok(body),
            false => Err(NFTStorageError::ApiError(body)),
        }
    }

    /// Check if the provided NFT cid is stored on nft storage
    ///
    /// It will check the nft by `cid`
    /// ```
    /// use nft_storage::NftStorage;
    /// use anyhow::Result;
    /// use serde_json::Value;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// get nft by cid
    /// 	let is_stored_on_nft_storage: Value  = nft_storage.check_nft("bafybeihflij24dndd6qo3aacbbysuzuygis7yurvrzxp3uk7bk5kfvfsfg").await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    pub async fn check_nft(&self, cid: &str) -> Result<Value, NFTStorageError> {
        let url = format!("{}/check/{}", self.url, cid);
        let response = self.client.get(url).bearer_auth(&self.token).send().await?;
        // check if the status of the request is in range of 200-299
        let status = response.status().is_success();
        let body = response.json::<Value>().await?;
        match status {
            true => Ok(body),
            false => Err(NFTStorageError::ApiError(body)),
        }
    }
}
