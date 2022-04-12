mod error;
mod types;
pub use crate::{error::NFTStorageError, types::*};
use anyhow::Result;
use reqwest::{
    multipart::{Form, Part},
    Client,
};
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
    /// The `url` is the url of the api which nftt storage is using for more information see https://nft.storage/api-docs/.
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
    pub fn new<S>(url: S, token: S) -> NftStorage
    where
        S: Into<String>,
    {
        NftStorage {
            client: Client::new(),
            url: url.into(),
            token: token.into(),
        }
    }

    /// List all nfts from nft storage
    /// `before` is used to return results created before provided timestamp `2021-12-01T08:52:33` or like this `2020-07-27T17:32:28Z` which is  and `limit` are the max records to return.
    ///
    /// the `only_metadata` option is used to return only the nft which contains the metadata.json file
    /// ```
    /// use nft_storage::{NftStorage, types::*}
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// list nfts only with metadata
    /// 	let list_nfts: ListNftResponse  = nft_storage.list_all_stored_nft(None, None, true).await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    ///
    pub async fn list_all_stored_nft(
        &self,
        before: Option<&str>,
        limit: Option<&str>,
        only_metadata: bool,
    ) -> Result<ListNftResponse, NFTStorageError> {
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
        let body = response.json::<Value>().await?;
        // if status is not success then return an error
        if !status {
            return Err(NFTStorageError::ApiError(body));
        }
        let mut body: ListNftResponse = serde_json::from_value(body)?;
        // if true get only metadata.json files and skip others
        if only_metadata {
            let final_filtered_list = body
                .value
                .into_iter()
                // we always know that there is only one file in the files array if we store a metadata nft
                .filter(|f| f.files.get(0).is_some() && f.files[0].name == "metadata.json")
                // add additional convenience links
                .map(|mut f| {
                    let link_1 = format!("https://{}.ipfs.dweb.link/metadata.json", f.cid);
                    let link_2 = format!("https://ipfs.io/ipfs/{}/metadata.json", f.cid);
                    let link_3 = format!("ipfs://{}/metadata.json", f.cid);

                    f.link = vec![link_1, link_2, link_3];

                    f
                })
                .collect::<Vec<_>>();
            // modify the body with modified data, does include only metadata.json
            body.value = final_filtered_list;
        } else {
            let final_filtered_list = body
                .value
                .into_iter()
                // add additional convenience links to the filtered
                .map(|mut f| {
                    let link_1 = format!("https://{}.ipfs.dweb.link", f.cid);
                    let link_2 = format!("https://ipfs.io/ipfs/{}", f.cid);
                    let link_3 = format!("ipfs://{}", f.cid);

                    f.link = vec![link_1, link_2, link_3];

                    f
                })
                .collect::<Vec<_>>();
            // modify the body with modified data, does include metadata.json and also all files
            body.value = final_filtered_list;
        }
        Ok(body)
    }

    /// Store an NFT on nft storage
    ///
    /// `file` is the file bytes recieved from a form-data
    ///
    /// `file_name` is the filename of the file we want to create an nft
    ///
    /// `nft_name` is the nft name and `description` is the description of the nft
    ///
    /// the main difference from `upload_file` method is that after uploading the file it creates a `metadata.json` file which
    /// contains the uploaded file cid and also the nft name and it's description
    ///
    /// ```
    /// use nft_storage::{NftStorage, types::*}
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// read a file in order to have a Vec<u8> the same from a form-data
    /// 	let file = std::fs::read("my_nft.jpg")?;
    /// 	// store an nft
    /// 	let store_nft: StoreNftResponse  = nft_storage.store_nft(file, "My NFT name", "My NFT description").await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    ///
    pub async fn store_nft<S>(
        &self,
        file: Vec<u8>,
        nft_name: S,
        description: S,
    ) -> Result<StoreNftResponse, NFTStorageError>
    where
        S: AsRef<str>,
    {
        // upload the file to nft storage, which is the actual file we want to create an nft
        let response: StoreNftResponse = self.upload_file(file).await?;
        // get dir cid
        let cid = response.value.cid;
        // create the metadata form which will contain all files cid
        let metadata = json!({
            "name": nft_name.as_ref(),
            "description": description.as_ref(),
            "files": cid
        });
        // create the form-data instance for metadata.json
        let metadata_json_bytes = serde_json::to_vec(&metadata)?;
        // create the metadata.json which will contain the nft cids
        let response: StoreNftResponse = self.upload_file(metadata_json_bytes).await?;
        Ok(response)
    }

    /// Delete an NFT
    ///
    /// `cid` is the ipfs hash, every file/nft has it's unique cid
    ///
    /// ```
    /// use nft_storage::{NftStorage, types::*}
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// delete nft by cid
    /// 	let delete_nft: DeleteNftResponse  = nft_storage.delete_nft("bafybeihflij24dndd6qo3aacbbysuzuygis7yurvrzxp3uk7bk5kfvfsdt").await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    ///
    pub async fn delete_nft<S>(&self, cid: S) -> Result<DeleteNftResponse, NFTStorageError>
    where
        S: AsRef<str>,
    {
        // create the url
        let url = format!("{}/{}", self.url, cid.as_ref());
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
            true => Ok(serde_json::from_value(body)?),
            false => Err(NFTStorageError::ApiError(body)),
        }
    }

    /// Delete all NFT
    ///
    /// ⚠ WARNING! ⚠
    ///
    /// It will fetch and delete all nfts
    ///
    /// This method is meant for developing purposes, it can be quite dangerous in production.
    /// ```
    /// use nft_storage::{NftStorage, types::*}
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// delete all nfts
    /// 	let delete_nfts: DeleteNftResponse  = nft_storage.delete_all_nft().await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    ///
    pub async fn delete_all_nft(&self) -> Result<DeleteNftResponse, NFTStorageError> {
        // create a loop to iterate and reqwest all nfts
        loop {
            // get first 100 nfts
            let nfts: ListNftResponse = self.list_all_stored_nft(None, Some("100"), false).await?;
            // check if ok is true this means the request was successfull and also check if the array is empty
            // if all is true break the loop this mean no nft are stored
            if nfts.ok && nfts.value.len() <= 0 {
                break;
            }
            for e in nfts.value {
                println!("deleting CID: {}", e.cid);
                self.delete_nft(&e.cid).await?;
                println!("NFT deleted: {}", e.cid);
            }
        }

        Ok(DeleteNftResponse { ok: true })
    }

    /// Retrive an NFT
    ///
    /// It will fetch an nft from `cid`
    /// ```
    /// use nft_storage::{NftStorage, types::*}
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// get nft by cid
    /// 	let get_nft: GetNftResponse  = nft_storage.get_nft("bafybeihflij24dndd6qo3aacbbysuzuygis7yurvrzxp3uk7bk5kfvfsfg").await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    ///
    pub async fn get_nft<S>(&self, cid: S) -> Result<GetNftResponse, NFTStorageError>
    where
        S: AsRef<str>,
    {
        let url = format!("{}/{}", self.url, cid.as_ref());
        let response = self.client.get(url).bearer_auth(&self.token).send().await?;
        // check if the status of the request is in range of 200-299
        let status = response.status().is_success();
        let body = response.json::<Value>().await?;
        if !status {
            return Err(NFTStorageError::ApiError(body));
        }
        let mut body: GetNftResponse = serde_json::from_value(body)?;
        // add some convinient links
        let link_1 = format!("https://{}.ipfs.dweb.link", body.value.cid);
        let link_2 = format!("https://ipfs.io/ipfs/{}", body.value.cid);
        let link_3 = format!("ipfs://{}", body.value.cid);
        // modify the body with added links
        body.value.link = vec![link_1, link_2, link_3];

        Ok(body)
    }

    /// Upload an arbitrary file on Nft storage
    ///
    /// It will upload an arbitrary file on ipfs backed up by nft storage and filecoin
    ///
    /// the max sise is around 30GB per file
    /// ```
    /// use nft_storage::{NftStorage, types::*}
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// read a file in order to have a Vec<u8> the same from a form-data
    /// 	let file = std::fs::read("my_nft.jpg")?;
    /// 	// delete nft
    /// 	let upload_file: StoreNftResponse  = nft_storage.upload_file(file).await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    pub async fn upload_file(&self, file: Vec<u8>) -> Result<StoreNftResponse, NFTStorageError> {
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
            true => Ok(serde_json::from_value(body)?),
            false => Err(NFTStorageError::ApiError(body)),
        }
    }

    /// Check if the provided NFT cid is stored on nft storage
    ///
    /// It will check the nft by `cid`
    /// ```
    /// use nft_storage::{NftStorage, types::*}
    /// use anyhow::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// get nft by cid
    /// 	let is_stored_on_nft_storage: CheckCidNftResponse  = nft_storage.check_nft("bafybeihflij24dndd6qo3aacbbysuzuygis7yurvrzxp3uk7bk5kfvfsfg").await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    pub async fn check_nft<S>(&self, cid: S) -> Result<CheckCidNftResponse, NFTStorageError>
    where
        S: AsRef<str>,
    {
        let url = format!("{}/check/{}", self.url, cid.as_ref());
        let response = self.client.get(url).bearer_auth(&self.token).send().await?;
        // check if the status of the request is in range of 200-299
        let status = response.status().is_success();
        let body = response.json::<Value>().await?;
        match status {
            true => Ok(serde_json::from_value(body)?),
            false => Err(NFTStorageError::ApiError(body)),
        }
    }

    /// Upload multiple files to Nft Storage
    ///
    /// It will upload multiple files using multipart form data and files will be stored in IPFS Directory
    ///
    /// The max sise is around 30GB per file
    ///
    /// The main difference between `upload_file` method is that it upload a file NOT in a direcotry so its url is unique, if using
    /// `upload_file_in_directory` it will store one or multiple files in an ipfs direcotry preserving the original filenames and all file will be
    /// fetched by the direcotry cid  for example `bafybeihflij24dndd6qo3aacbbysuzuygis7yurvrzxp3uk7bk5kfvfsfg/my_file.txt`
    ///
    /// Every time using this method it will create a new directory
    /// ```
    /// use anyhow::Result;
    /// use nft_storage::{NftStorage, types::*}
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// read a file in order to have a Vec<u8> the same from a form-data
    /// 	let file = std::fs::read("my_nft.jpg")?;
    /// 	let fil2 = std::fs::read("my_nft2.jpg")?;
    /// 	// create a vec of files bytes
    /// 	let v = vec![file, file2];
    /// 	// create a vec of file names
    /// 	let f = vec!["my_nft.jpg".to_String()., "my_nft2.jpg".to_string()];
    /// 	// delete nft
    /// 	let upload_file: StoreNftResponse  = nft_storage.upload_file_in_directory(v, f).await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    pub async fn upload_file_in_directory<S>(
        &self,
        files: Vec<Vec<u8>>,
        file_names: Vec<S>,
    ) -> Result<StoreNftResponse, NFTStorageError>
    where
        S: AsRef<str>,
    {
        let url = format!("{}/upload", self.url);
        let mut form = Form::new();
        // creating a custom part of teh form
        for (index, _) in files.iter().enumerate() {
            let part =
                Part::bytes(files[index].clone()).file_name(file_names[index].as_ref().to_string());
            form = Form::from(form.part("file", part));
        }
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.token)
            .multipart(form)
            .send()
            .await?;
        let status = response.status().is_success();
        // check if the status of the request is in range of 200-299
        let body = response.json::<Value>().await?;
        match status {
            true => Ok(serde_json::from_value(body)?),
            false => Err(NFTStorageError::ApiError(body)),
        }
    }

    /// Store an NFT on nft storage in a directory
    ///
    /// `file` is the array containing the the file bytes vecs recieved from a form-data
    ///
    /// `file_name` is the filename of the file we want to create an nft
    ///
    /// `nft_name` is the nft name and `description` is the description of the nft
    ///
    /// The difference from `upload_file_in_directory` method is that after uploading all files it creates a `metadata.json` file
    /// that lists all files uploaded and also assigns the nft name and it's description, this metadata.json file it is stored on a IPFS Direcotry
    ///
    /// ```
    /// use anyhow::Result;
    /// use nft_storage::{NftStorage, types::*}
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    /// 	// provide the url and as second argument the token generated from nft storage dashboard
    /// 	let nft_storage = NftStorage::new("https://api.nft.storage", "token generated from nft storage");
    /// 	// read a file in order to have a Vec<u8> the same from a form-data
    /// 	let file = std::fs::read("my_nft.jpg")?;
    /// 	let fil2 = std::fs::read("my_nft2.jpg")?;
    /// 	// create a vec of file bytes
    /// 	let v = vec![file, file2];
    /// 	// store an nft
    /// 	let store_nft: StoreNftResponse  = nft_storage.store_nft_in_directory(v, "My NFT name", "My NFT description").await?;
    ///
    /// 	Ok(())
    /// }
    /// ```
    ///
    pub async fn store_nft_in_directory<S>(
        &self,
        files: Vec<Vec<u8>>,
        file_names: Vec<S>,
        nft_name: S,
        description: S,
    ) -> Result<StoreNftResponse, NFTStorageError>
    where
        S: AsRef<str>,
    {
        // upload the file to nft storage, which is the actual file we want to create an nft
        let response = self.upload_file_in_directory(files, file_names).await?;
        // get value array
        let value = response.value.files;
        // get cid of the folder that contains uploaded files
        let cid = response.value.cid;

        // get all filenames uploaded
        let mut file_names = value.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
        // concatenate to create the ipfs link to paste in metadata.json
        let file_cids = file_names
            .iter_mut()
            .map(|f| format!("ipfs://{}/{}", cid, f))
            .collect::<Vec<_>>();
        // create athe metadata form which will contain all files cid
        let metadata = json!({
            "name": nft_name.as_ref(),
            "description": description.as_ref(),
            "files": file_cids
        });
        // create the form-data instance for metadata.json
        let metadata_json_bytes = serde_json::to_vec(&metadata)?;
        // create the metadata.json which will contain the nft cids
        let response = self
            .upload_file_in_directory(vec![metadata_json_bytes], vec!["metadata.json".to_string()])
            .await?;

        Ok(response)
    }
}
