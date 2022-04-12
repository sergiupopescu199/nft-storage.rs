use serde::{Deserialize, Serialize};

/// list nft response from nft storage api
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ListNftResponse {
    /// status of the request
    pub ok: bool,
    /// all nft data
    pub value: Vec<Value>,
}

/// response after an nft was stored
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StoreNftResponse {
    /// status of the request
    pub ok: bool,
    /// stored nft data
    pub value: Value,
}

/// query nft from nft storage api
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GetNftResponse {
    /// status of the request
    pub ok: bool,
    /// queried nft data
    pub value: Value,
}

/// response of a deleted nft
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeleteNftResponse {
    /// status of the request
    pub ok: bool,
}

/// check if an nft exist response from nft storage api
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CheckCidNftResponse {
    /// status of the request
    pub ok: bool,
    /// data of nft
    pub value: CheckNFTValue,
}

/// main obj that hold all the response data
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Value {
    /// ipfs cid (file hash)
    pub cid: String,
    /// file size
    pub size: i32,
    /// date uploaded
    pub created: String,
    /// type of the file (mime type)
    pub file_type: String,
    pub scope: String,
    /// filecoin pin data
    pub pin: Pin,
    /// file data (name and mime type)
    pub files: Vec<Files>,
    /// filecoin deals data
    pub deals: Vec<Deals>,
    /// ipfs links to view file
    pub link: Vec<String>,
}

/// data that holds data about queried nft when checking when it exists on nft storage
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CheckNFTValue {
    /// ipfs cid (file hash)
    pub cid: String,
    /// filecoin pin data
    pub pin: Pin,
    /// filecoin deals data
    pub deals: Vec<Deals>,
}

/// filecoin pin data
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Pin {
    /// ipfs cid (file hash)
    pub cid: String,
    /// pin status of the nft
    pub status: String,
    /// creation date
    pub created: String,
    /// size of teh file
    pub size: i32,
}

/// file information
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Files {
    /// file name
    pub name: String,
    /// file mime type
    pub file_type: String,
}

/// filecoin deals data
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Deals {
    #[serde(rename = "batchRootCid")]
    pub batch_root_cid: String,
    #[serde(rename = "lastChanged")]
    pub last_changed: String,
    pub miner: String,
    #[serde(rename = "pieceCid")]
    pub piece_cid: String,
    pub status: String,
    #[serde(rename = "statusText")]
    pub status_text: String,
    #[serde(rename = "chainDealID")]
    pub chain_deal_id: i32,
    #[serde(rename = "dealActivation")]
    pub deal_activation: String,
    #[serde(rename = "dealExpiration")]
    pub deal_expiration: String,
    #[serde(rename = "datamodelSelector")]
    pub data_model_selector: String,
}
