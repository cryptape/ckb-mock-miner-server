//! Example server with macros.

mod mock_data;
use clap::{App, Arg};

use jsonrpc_core::{Error, ErrorCode, Value};

use std::{sync::Arc};
use std::sync::Mutex;
use async_trait::async_trait;
use ckb_jsonrpc_types::{Block, BlockTemplate, Uint64, Version};
use ckb_types::{H256};
use jsonrpc_core::{MetaIoHandler, Result};
use jsonrpc_utils::{
    axum_utils::jsonrpc_router, rpc, stream::StreamServerConfig,
};


use crate::mock_data::{get_mock_data_by_path, MockData};


#[rpc]
#[async_trait]
trait MinerMockRpc {
    #[rpc(name = "submit_block")]
    fn submit_block(&self, work_id: String, block: Block) -> Result<H256>;
    #[rpc(name = "get_block_template")]
    fn get_block_template(
        &self,
        bytes_limit: Option<Uint64>,
        proposals_limit: Option<Uint64>,
        max_version: Option<Version>,
    ) -> Result<BlockTemplate>;
}

#[derive(Clone)]
pub struct MinerMockRpcImpl {
    idx: Arc<Mutex<usize>>,
    mock_data: Vec<MockData>,
}

#[async_trait]
impl MinerMockRpc for MinerMockRpcImpl {
    fn submit_block(&self, work_id: String, block: Block) -> Result<H256> {
        // todo check work id ?
        let latest_idx = self.idx.lock().unwrap().clone();
        let idx = latest_idx % self.mock_data.len();
        return match self.mock_data.get(idx).unwrap().verify_submit_block(block.clone()) {
            Ok(hash) => {
                *self.idx.lock().unwrap() = latest_idx + 1;
                println!("submit successful:{}", block.header.number.clone());
                Ok(hash)
            }
            Err(err) => {
                Err(Error {
                    code: ErrorCode::ServerError(1i64),
                    message: format!("{err:?}"),
                    data: Some(Value::String(format!("{err:?}"))),
                })
            }
        };
    }

    fn get_block_template(&self, bytes_limit: Option<Uint64>, proposals_limit: Option<Uint64>, max_version: Option<Version>) -> Result<BlockTemplate> {
        let current_idx = self.idx.lock().unwrap().clone();
        println!("== get_block_template :{} ",current_idx);
        let idx = current_idx % self.mock_data.len();
        return Ok(self.mock_data.get(idx).unwrap().get_block_temple.clone());
    }
}


#[tokio::main]
async fn main() {
    let matches = App::new("JSON-RPC Mock Server")
        .arg(Arg::with_name("bind")
            .short("b")
            .long("bind")
            .default_value("0.0.0.0:3000")
            .help("The address to bind the server to")
        )
        .arg(Arg::with_name("mock_data_path")
            .short("p")
            .long("mock-data-path")
            .default_value("./mock.json")
            .help("Path to the mock data JSON file"))

        .get_matches();
    let bind_addr = matches.value_of("bind").unwrap();
    let mock_data_path = matches.value_of("mock_data_path").unwrap();

    let mut rpc = MetaIoHandler::with_compatibility(jsonrpc_core::Compatibility::V2);
    let rpc_impl = MinerMockRpcImpl {
        idx: Arc::new(Mutex::new(1)),
        mock_data: get_mock_data_by_path(mock_data_path.to_string()),
    };
    add_miner_mock_rpc_methods(&mut rpc, rpc_impl);
    let rpc = Arc::new(rpc);
    let stream_config = StreamServerConfig::default().with_keep_alive(true);

    let app = jsonrpc_router("/", rpc, stream_config);
    axum::Server::bind(&bind_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}