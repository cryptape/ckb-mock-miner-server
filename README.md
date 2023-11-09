# ckb-mock-miner
miner mock rpc server 


start ckb mock server 
```shell
cargo build 
target/debug/ckb-mock-miner-server --bind 0.0.0.0:3000 --mock-data-path ./mock.json
```

test server start 
```shell
curl --location 'localhost:3000/' \
--header 'Content-Type: application/json' \
--data '{
        "jsonrpc":"2.0",
        "method":"get_block_template",
        "params":[],                          
        "id":64
}' 
```

### test case (./mock.json)
- [x] extension in dev 
- [ ] 设计一个挖矿奖励减半后的用例
- [ ] 补充一个 hardfork 的测试用例，block version 不为 0

### verify rule 
```shell
1. verify extra_hash
2. verify proposals_hash
3. verify transactions_root
4. verify  nonce
```

