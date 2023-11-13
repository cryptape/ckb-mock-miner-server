use std::fs::File;
use std::io::{Read};
use ckb_jsonrpc_types::{Block, BlockTemplate};
use ckb_types::{core, packed, prelude::*, H256};
use ckb_pow::{Pow};

use std::sync::Arc;
use ckb_jsonrpc_types::ResponseFormatInnerType::Json;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MockData {
    pub case: String,
    pub get_block_temple: BlockTemplate,
    pub submit_block: Block,
}

impl MockData {
    pub fn verify_submit_block(&self, block: Block) -> Result<H256, String> {
        let block_data: packed::Block = block.clone().into();
        let block_view: Arc<core::BlockView> = Arc::new(block_data.into_view());
        let header_view = block_view.header();

        // verify extra_hash
        if block.header.extra_hash != self.submit_block.header.extra_hash {
            return Err(format!("verify extra_hash failed,submit block : {:?} !=expected block : {:?}", serde_json::to_string(&block.clone()), serde_json::to_string(&self.submit_block.clone())));
        }

        if header_view.extra_hash().unpack() != self.submit_block.header.extra_hash {
            return Err(format!("verify calc extra_hash failed,submit block : {:?} !=expected block : {:?},", serde_json::to_string(&block.clone()), serde_json::to_string(&self.submit_block.clone())));
        }


        // verify proposals_hash
        if block.header.proposals_hash != self.submit_block.header.proposals_hash {
            return Err(format!("verify proposals_hash failed,submit block : {:?} !=expected block : {:?},", serde_json::to_string(&block.clone()), serde_json::to_string(&self.submit_block.clone())));
        }

        if header_view.proposals_hash().unpack() != self.submit_block.header.proposals_hash {
            return Err(format!("verify calc  proposals_hash failed,submit block : {:?} !=expected block : {:?},", serde_json::to_string(&block.clone()), serde_json::to_string(&self.submit_block.clone())));
        }

        // verify transactions_root
        if block.header.transactions_root != self.submit_block.header.transactions_root {
            return Err(format!("verify transactions_root failed,submit block : {:?} !=expected block : {:?},", serde_json::to_string(&block.clone()), serde_json::to_string(&self.submit_block.clone())));
        }

        if header_view.transactions_root().unpack() != self.submit_block.header.transactions_root {
            return Err(format!("verify calc transactions_root failed,submit block : {:?} !=expected block : {:?},", serde_json::to_string(&block.clone()), serde_json::to_string(&self.submit_block.clone())));
        }

        // verify  nonce
        if !Pow::Eaglesong.engine().verify(&header_view.data()) {
            return Err(format!("verify nonce use Eaglesong  failed,submit block : {:?} !=expected block : {:?},", serde_json::to_string(&block.clone()), serde_json::to_string(&self.submit_block.clone())));
        }
        return Ok(header_view.hash().unpack());
    }
}

pub fn get_mock_data_by_path(file_path: String) -> Vec<MockData> {
    if file_path == "" {
        return Vec::new();
    }
    let mut file = File::open(file_path).expect("not found file path");
    let mut json_content = String::new();
    file.read_to_string(&mut json_content).expect("Failed to read the file");
    let deserialized_object: Vec<MockData> = serde_json::from_str(&json_content).expect("Failed to deserialize JSON");
    deserialized_object
}

#[test]
pub fn test_block_temple() {
    let mds = get_mock_data_by_path("./mock.json".to_string());
    match mds.get(0).unwrap().verify_submit_block(mds.get(0).unwrap().submit_block.clone()) {
        Ok(hash) => {
            println!("hash:{}", hash)
        }
        Err(err) => {
            println!("err:{}", err)
        }
    }
}



